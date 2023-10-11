import { near, JSONValue, json, ipfs, log, TypedMap, Value, typeConversion, BigDecimal, BigInt } from "@graphprotocol/graph-ts"
import { 
  Proposal,
  //Proponent
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

  if (methodName == 'set_proposal') {
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
          proposal.creation_date = creation_date.toString();
          proposal.user_creation = user_creation.toString();
          proposal.link = link.toString()
          proposal.save()
        }
        
      }
    }
  }

}