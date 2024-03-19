import { near } from "@graphprotocol/graph-ts";

import DaoMeta from "./lib/dao_meta";
import NearToken from "./lib/near_token";

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
  DaoMeta(action, receipt, outcome, blockHeader);
  
  NearToken(action, receipt, blockHeader);
}
