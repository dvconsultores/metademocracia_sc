import { near, JSONValue, json, ipfs, log, TypedMap, Value, typeConversion, BigDecimal, BigInt } from "@graphprotocol/graph-ts"
import {
  Delegation,
  Delegationhist,
  Delegator,
  Fundshist,
} from "../generated/schema"
import { walletRastreo } from "./config"

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

const token_id: string = "USDT";
//const walletRastreo = "daov1.metademocracia_dao.near";

function handleAction(
  action: near.ActionValue,
  receipt: near.ActionReceipt,
  outcome: near.ExecutionOutcome,
  blockHeader: near.BlockHeader
): void {
  if (action.kind != near.ActionKind.FUNCTION_CALL) return;

  const methodName = action.toFunctionCall().methodName;
  
  if (methodName == 'ft_transfer' || methodName == "ft_transfer_call") {  
    if(outcome.logs.length > 0) {
      for (let index = 0; index < outcome.logs.length; index++) {
        //obtenemos la primera iteracion del log
        const outcomeLog = outcome.logs[index].toString();

        if(!outcomeLog.includes('EVENT_JSON:')) return

        const parsed = outcomeLog.replace('EVENT_JSON:', '')
        
        //convirtiendo el log en un objeto ValueJSON
        let outcomelogs = json.try_fromString(parsed);
        
        //validamos que se cree un objeto tipo ValueJSON valido a partir del log capturado
        if(!outcomelogs.isOk) return;

        const jsonlog = outcomelogs.value.toObject();
        const eventData = jsonlog.get('data')
        
        if (!eventData) return
        
        const eventArray:JSONValue[] = eventData.toArray()

        const data = eventArray[0].toObject()
        const new_owner_id = data.get('new_owner_id');
        const amount = data.get('amount');
        const old_owner_id = data.get('old_owner_id');

        
        if (!new_owner_id || !amount || !old_owner_id) return
        
        //entrada
        if(new_owner_id.toString() == walletRastreo) {
          let delegation = Delegation.load(token_id);
          if (!delegation) {
            delegation = new Delegation(token_id);
            delegation.total_amount = BigInt.fromI32(0);
          }

          delegation.total_amount = delegation.total_amount.plus(BigInt.fromString(amount.toString()));

          delegation.save()
          
          let id_delegationhist = token_id + old_owner_id.toString() + BigInt.fromU64(blockHeader.timestampNanosec).toString()
          let delegationhist = Delegationhist.load(id_delegationhist);
          if (!delegationhist) {
            delegationhist = new Delegationhist(id_delegationhist);
            delegationhist.delegation = token_id;
            delegationhist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
            delegationhist.delegator = old_owner_id.toString();
            delegationhist.amount = BigInt.fromString(amount.toString());

            delegationhist.save();
          }
          
          let id_funds = token_id + old_owner_id.toString() + BigInt.fromU64(blockHeader.timestampNanosec).toString()
          let fundshist = new Fundshist(id_funds);
          fundshist.user_id = old_owner_id.toString()
          fundshist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
          fundshist.token_id = token_id;
          fundshist.amount = BigDecimal.fromString(amount.toString()).div(BigDecimal.fromString("1000000"));
          fundshist.type = "received";

          fundshist.save();

          let id_delegator = token_id + old_owner_id.toString()
          let delegator_entity = Delegator.load(id_delegator);
          if (!delegator_entity) {
            delegator_entity = new Delegator(id_delegator);
            delegator_entity.delegation = token_id;
            delegator_entity.delegator = old_owner_id.toString();
            delegator_entity.amount = BigInt.fromI32(0);
          }

          delegator_entity.amount = delegator_entity.amount.plus(BigInt.fromString(amount.toString()));

          delegator_entity.save();
        
        }

        //salida
        if(old_owner_id.toString() == walletRastreo) {
          let delegation = Delegation.load(token_id);
          if (!delegation) {
            delegation = new Delegation(token_id);
            delegation.total_amount = BigInt.fromI32(0);
          }

          delegation.total_amount = delegation.total_amount.minus(BigInt.fromString(amount.toString()));

          delegation.save()
          
          /*let id_delegationhist = token_id + old_owner_id.toString() + BigInt.fromU64(blockHeader.timestampNanosec).toString()
          let delegationhist = Delegationhist.load(id_delegationhist);
          if (!delegationhist) {
            delegationhist = new Delegationhist(id_delegationhist);
            delegationhist.delegation = token_id;
            delegationhist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
            delegationhist.delegator = old_owner_id.toString();
            delegationhist.amount = BigInt.fromString(amount.toString());

            delegationhist.save();
          }*/
          
          let id_funds = token_id + old_owner_id.toString() + BigInt.fromU64(blockHeader.timestampNanosec).toString()
          let fundshist = new Fundshist(id_funds);
          fundshist.user_id = new_owner_id.toString()
          fundshist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
          fundshist.token_id = token_id;
          fundshist.amount = BigDecimal.fromString(amount.toString()).div(BigDecimal.fromString("1000000"));
          fundshist.type = "transfer";

          fundshist.save();

          /*let id_delegator = token_id + old_owner_id.toString()
          let delegator_entity = Delegator.load(id_delegator);
          if (!delegator_entity) {
            delegator_entity = new Delegator(id_delegator);
            delegator_entity.delegation = token_id;
            delegator_entity.delegator = old_owner_id.toString();
            delegator_entity.amount = BigInt.fromI32(0);
          }

          delegator_entity.amount = delegator_entity.amount.plus(BigInt.fromString(amount.toString()));

          delegator_entity.save();*/
        
        }

      }
    }
  }

  
}