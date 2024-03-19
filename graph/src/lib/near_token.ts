import {
  near,
  JSONValue,
  json,
  ipfs,
  log,
  TypedMap,
  Value,
  typeConversion,
  BigDecimal,
  BigInt,
  Bytes,
  JSONValueKind,
} from "@graphprotocol/graph-ts";

import {   
  Delegation,
  Delegationhist,
  Delegator,
  Fundshist 
} from "../../generated/schema";

// import { includeAcount } from "./config";
import { walletRastreo } from "../config"

export default function nearToken(
  action: near.ActionValue,
  receipt: near.ActionReceipt,
  blockHeader: near.BlockHeader
): void {
  if (action.kind != near.ActionKind.TRANSFER) return;

  const deposit = action.toTransfer().deposit;
  const id_delegation = receipt.id.toBase58();

  let delegation = Delegation.load("NEAR");
  if (!delegation) {
    delegation = new Delegation("NEAR");
    delegation.total_amount = BigInt.fromI32(0);
    delegation.save()
  }

  //entradas
  if (
    receipt.receiverId.includes(walletRastreo) &&
    receipt.predecessorId != "system"
  ) {

    delegation.total_amount = delegation.total_amount.plus(BigInt.fromString(deposit.toString()));
    delegation.save()

    let id_delegationhist = "NEAR" + receipt.receiverId + BigInt.fromU64(blockHeader.timestampNanosec).toString()
    let delegationhist = Delegationhist.load(id_delegationhist);
    if (!delegationhist) {
      delegationhist = new Delegationhist(id_delegationhist);
      delegationhist.delegation = "NEAR";
      delegationhist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
      delegationhist.delegator = receipt.receiverId;
      delegationhist.amount = BigInt.fromString(deposit.toString());

      delegationhist.save();
    }

    let id_funds = "NEAR" + receipt.receiverId + BigInt.fromU64(blockHeader.timestampNanosec).toString()
    let fundshist = new Fundshist(id_funds);
    fundshist.user_id = receipt.receiverId;
    fundshist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
    fundshist.token_id = "NEAR";
    fundshist.amount = BigDecimal.fromString(deposit.toString()).div(BigDecimal.fromString("1000000000000000000000000"));
    fundshist.type = "received";

    fundshist.save();

    let id_delegator = "NEAR" + receipt.receiverId
    let delegator_entity = Delegator.load(id_delegator);
    if (!delegator_entity) {
      delegator_entity = new Delegator(id_delegator);
      delegator_entity.delegation = "NEAR";
      delegator_entity.delegator = receipt.receiverId;
      delegator_entity.amount = BigInt.fromI32(0);
    }

    delegator_entity.amount = delegator_entity.amount.plus(BigInt.fromString(deposit.toString()));

    delegator_entity.save();
  }

  //salidas
  if(receipt.predecessorId.includes(walletRastreo)){
    delegation.total_amount = delegation.total_amount.minus(BigInt.fromString(deposit.toString()));
    delegation.save()

    let id_funds = "NEAR" + receipt.predecessorId.toString() + BigInt.fromU64(blockHeader.timestampNanosec).toString()
    let fundshist = new Fundshist(id_funds);
    fundshist.user_id = receipt.predecessorId.toString()
    fundshist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
    fundshist.token_id = "NEAR";
    fundshist.amount = BigDecimal.fromString(deposit.toString()).div(BigDecimal.fromString("1000000000000000000000000"));
    fundshist.type = "transfer";

    fundshist.save();
  }

  
    

      
        /*const prev_amount = jsonObject.get('prev_amount');
        const new_amount = jsonObject.get('new_amount');
        const delegate_amount = jsonObject.get('delegate_amount');
        const delegacion_total = jsonObject.get('delegacion_total');
        const delegator = jsonObject.get('delegator');

        if (!prev_amount || !new_amount || !delegate_amount || !delegacion_total || !delegator) return;
        
        let delegation = Delegation.load("NEAR");
        if (!delegation) {
          delegation = new Delegation("NEAR");
          delegation.total_amount = BigInt.fromI32(0);
        }

        delegation.total_amount = delegation.total_amount.plus(BigInt.fromString(delegate_amount.toString()));

        delegation.save()
        
        let id_delegationhist = "NEAR" + delegator.toString() + BigInt.fromU64(blockHeader.timestampNanosec).toString()
        let delegationhist = Delegationhist.load(id_delegationhist);
        if (!delegationhist) {
          delegationhist = new Delegationhist(id_delegationhist);
          delegationhist.delegation = "NEAR";
          delegationhist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
          delegationhist.delegator = delegator.toString();
          delegationhist.amount = BigInt.fromString(delegate_amount.toString());

          delegationhist.save();
        }

        let id_funds = "NEAR" + delegator.toString() + BigInt.fromU64(blockHeader.timestampNanosec).toString()
        let fundshist = new Fundshist(id_funds);
        fundshist.user_id = delegator.toString()
        fundshist.date_time = BigInt.fromU64(blockHeader.timestampNanosec);
        fundshist.token_id = "NEAR";
        fundshist.amount = BigDecimal.fromString(delegate_amount.toString()).div(BigDecimal.fromString("1000000000000000000000000"));
        fundshist.type = "received";

        fundshist.save();

        let id_delegator = "NEAR" + delegator.toString()
        let delegator_entity = Delegator.load(id_delegator);
        if (!delegator_entity) {
          delegator_entity = new Delegator(id_delegator);
          delegator_entity.delegation = "NEAR";
          delegator_entity.delegator = delegator.toString();
          delegator_entity.amount = BigInt.fromI32(0);
        }

        delegator_entity.amount = delegator_entity.amount.plus(BigInt.fromString(delegate_amount.toString()));

        delegator_entity.save(); */


        
        
      
    
  

}
