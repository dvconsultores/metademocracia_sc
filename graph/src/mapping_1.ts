import { near, JSONValue, json, ipfs, log, TypedMap, Value, typeConversion, BigDecimal, BigInt } from "@graphprotocol/graph-ts"
import { 
  Proposaltype,
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

  if (methodName == 'set_type_proposal') {
    if(outcome.logs.length > 0) {
      const outcomeLog = outcome.logs[0].toString();
      
      if(!json.try_fromString(outcomeLog).isOk) return
      let outcomelogs = json.try_fromString(outcomeLog);
      const jsonObject = outcomelogs.value.toObject();

      if (jsonObject) {
        const id = jsonObject.get('id');
        const name = jsonObject.get('name');
        
        if (!id || !name) return;
        
        let proposaltype = Proposaltype.load(id.toString())
        if (!proposaltype) {
          proposaltype = new Proposaltype(id.toString())
          proposaltype.name = name.toString()
          proposaltype.save()
        }
        
      }
    }
  }

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
        //const proponents = jsonObject.get('proponents');
        const proponent = jsonObject.get('proponent');
        const target = jsonObject.get('target');
        const time_complete = jsonObject.get('time_complete');
        const claims_available = jsonObject.get('claims_available');
        const amount = jsonObject.get('amount');
        const status = jsonObject.get('status');
        const creation_date = jsonObject.get('creation_date');
        const user_creation = jsonObject.get('user_creation');
        const link = jsonObject.get('link');
        
        if (!id || !title || !description || !proponent || !time_complete || !claims_available || !amount || !status || !creation_date || !user_creation || !link) return;
        
        let proposal = Proposal.load(id.toString());
        if (!proposal) {
          proposal = new Proposal(id.toString());
          proposal.title = title.toString();
          proposal.description = description.toString();
          proposal.time_to_complete = time_complete.toI64() as i32;
          //proposal.Claims_available = BigInt.fromString(claims_available.toString());
          proposal.proponent = proponent.toString()
          proposal.target = target!.toString()
          proposal.amount = BigInt.fromString(amount.toString());
          proposal.status = status.toI64() as i32;
          proposal.creation_date = creation_date.toString()
          proposal.user_creation = user_creation.toString()
          proposal.link = link.toString()
          proposal.save()
        }

        /*proponents.toArray().forEach((item) => {
          let proponent_id = item.toString() + "|" + id.toString() 
          let proponent = Proponent.load(id.toString());
          if (!proponent) {
            proponent = new Proposal(id.toString());
          }
        }) */
        
      }
    }
  }

}