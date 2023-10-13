import { near, JSONValue, json, ipfs, log, TypedMap, Value, typeConversion, BigDecimal, BigInt } from "@graphprotocol/graph-ts"
import {
  Proposaldata,
  Proposal,
  Vote,
  Delegation,
  Delegationhist,
  Delegator,
} from "../generated/schema"


export function handleReceipt(receipt: near.ReceiptWithOutcome): void {
  const actions = receipt.receipt.actions;
  for (let i = 0; i < actions.length; i++) {
    handleAction(
      actions[i], 
      receipt.receipt, 
      receipt.outcome,
      receipt.block.header
    );
  }
}


function handleAction(
  action: near.ActionValue,
  receipt: near.ActionReceipt,
  outcome: near.ExecutionOutcome,
  blockHeader: near.BlockHeader
): void {
  if (action.kind != near.ActionKind.FUNCTION_CALL) return;

  const methodName = action.toFunctionCall().methodName;

  if (methodName == 'on_set_proposal') {
    if(outcome.logs.length > 0) {
      const outcomeLog = outcome.logs[0].toString();
      
      if(!json.try_fromString(outcomeLog).isOk) return
      let outcomelogs = json.try_fromString(outcomeLog);
      const jsonObject = outcomelogs.value.toObject();
      

      if (jsonObject) {  
        const id = jsonObject.get('id');
        const title = jsonObject.get('title');
        const description = jsonObject.get('description');
        const proposal_type = jsonObject.get('proposal_type');
        const kind = jsonObject.get('kind');
        const proposer = jsonObject.get('proposer');
        const submission_time = jsonObject.get('submission_time');
        const status = jsonObject.get('status');
        const creation_date = jsonObject.get('creation_date');
        const user_creation = jsonObject.get('user_creation');
        const link = jsonObject.get('link');
        

        if (!id || !title || !description || !proposal_type || !kind || !proposer || !submission_time 
            || !status || !creation_date || !user_creation || !link) return;
        

        let proposal = Proposal.load(id.toString());
        if (!proposal) {
          proposal = new Proposal(id.toString());
          proposal.title = title.toString();
          proposal.description = description.toString();
          proposal.proposal_type = proposal_type.toString();
          proposal.kind = kind.toString();
          proposal.submission_time = BigInt.fromString(submission_time.toString());
          proposal.upvote = BigInt.fromI32(0);
          proposal.downvote = BigInt.fromI32(0);
          proposal.proposer = proposer.toString();
          proposal.status = status.toString();
          proposal.creation_date = BigInt.fromString(creation_date.toString());
          proposal.user_creation = user_creation.toString();
          proposal.link = link.toString()
          proposal.save()

          let proposaldata = Proposaldata.load("1");
          if(!proposaldata) {
            proposaldata = new Proposaldata("1")
            proposaldata.proposal_actives = BigInt.fromI32(0);
            proposaldata.proposal_total = BigInt.fromI32(0);
          }
          proposaldata.proposal_actives = proposaldata.proposal_actives.plus(BigInt.fromI32(1));
          proposaldata.proposal_total = proposaldata.proposal_total.plus(BigInt.fromI32(1));

          proposaldata.save();

        }
        
      }
    }
  }


  if (methodName == 'on_update_proposal') {
    if(outcome.logs.length > 0) {
      const outcomeLog = outcome.logs[0].toString();
      
      if(!json.try_fromString(outcomeLog).isOk) return
      let outcomelogs = json.try_fromString(outcomeLog);
      const jsonObject = outcomelogs.value.toObject();
      

      if (jsonObject) {  
        const id = jsonObject.get('id');
        const type = jsonObject.get('type');
        const action = jsonObject.get('action');
        const status = jsonObject.get('status');
        const memo = jsonObject.get('memo');
        const sender_id = jsonObject.get('sender_id');

        if (!id || !type || !action || !status || !memo || !sender_id) return;
        
        if(type.toString() == "vote") {
          let idVote = id.toString() + sender_id.toString();
          let vote = Vote.load(idVote);

          if(!vote) {
            vote = new Vote(idVote);
            vote.proposal = id.toString();
            vote.user_id = sender_id.toString();
            vote.vote = action.toString();

            let proposal = Proposal.load(id.toString());
            if (proposal) {
              if(action.toString() == "VoteApprove") {
                proposal.upvote = proposal.upvote.plus(BigInt.fromI32(1));
              }

              if(action.toString() == "VoteReject") {
                proposal.downvote = proposal.downvote.plus(BigInt.fromI32(1));
              }
              proposal.save();
            }

            if(status.toString() != "InProgress" && status.toString() != "Failed") {
              let proposaldata = Proposaldata.load("1");
              if(proposaldata) {
                proposaldata.proposal_actives = proposaldata.proposal_actives.minus(BigInt.fromI32(1));

                proposaldata.save();
              }
            }
            
            
            vote.save();
          }
          
        }
        
      }
    }
  }


  if (methodName == 'delegate') {
    if(outcome.logs.length > 0) {
      const outcomeLog = outcome.logs[0].toString();
      
      if(!json.try_fromString(outcomeLog).isOk) return
      let outcomelogs = json.try_fromString(outcomeLog);
      const jsonObject = outcomelogs.value.toObject();

      if (jsonObject) {  
        const prev_amount = jsonObject.get('prev_amount');
        const new_amount = jsonObject.get('new_amount');
        const delegate_amount = jsonObject.get('delegate_amount');
        const delegacion_total = jsonObject.get('delegacion_total');
        const delegator = jsonObject.get('delegator');

        if (!prev_amount || !new_amount || !delegate_amount || !delegacion_total || !delegator) return;
        
        let delegation = Delegation.load("near");
        if (!delegation) {
          delegation = new Delegation("near");
          delegation.total_amount = BigInt.fromI32(0);
        }

        delegation.total_amount = delegation.total_amount.plus(BigInt.fromString(delegate_amount.toString()));

        delegation.save()
        
        let id_delegationhist = "near" + delegator.toString() + BigInt.fromU64(blockHeader.timestampNanosec).toString()
        let delegationhist = Delegationhist.load(id_delegationhist);
        if (!delegationhist) {
          delegationhist = new Delegationhist(id_delegationhist);
          delegationhist.delegation = "near";
          delegationhist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
          delegationhist.delegator = delegator.toString();
          delegationhist.amount = BigInt.fromString(delegate_amount.toString());

          delegationhist.save();
        }

        let id_delegator = "near" + delegator.toString()
        let delegator_entity = Delegator.load(id_delegator);
        if (!delegator_entity) {
          delegator_entity = new Delegator(id_delegator);
          delegator_entity.delegation = "near";
          delegator_entity.delegator = delegator.toString();
          delegator_entity.amount = BigInt.fromI32(0);
        }

        delegator_entity.amount = delegator_entity.amount.plus(BigInt.fromString(delegate_amount.toString()));

        delegator_entity.save();


        
        
      }
    }
  }


}