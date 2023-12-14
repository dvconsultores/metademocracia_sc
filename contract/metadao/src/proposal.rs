use crate::*;
use near_contract_standards::fungible_token::core_impl::ext_fungible_token;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[derive(Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Action {
    AddProposal,
    RemoveProposal,
    VoteApprove,
    VoteReject,
    VoteRemove,
    Finalize,
}

impl Action {
    pub fn to_label(&self) -> &str {
      match self {
        Action::AddProposal { .. } => "AddProposal",
        Action::RemoveProposal { .. } => "RemoveProposal",
        Action::VoteApprove { .. } => "VoteApprove",
        Action::VoteReject { .. } => "VoteReject",
        Action::VoteRemove { .. } => "VoteRemove",
        Action::Finalize { .. } => "Finalize",
      }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
// #[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[derive(Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ActionCall {
    method_name: String,
    args: Base64VecU8,
    deposit: U128,
    gas: U64,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    InProgress,
    Approved,
    Rejected,
    Removed,
    Expired,
    Failed,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PolicyParameters {
  pub proposal_bond: Option<U128>,
  pub proposal_period: Option<U64>,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
// #[cfg_attr(not(target_arch = "wasm32"))]
#[derive(Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalKind {

    ChangePolicy { policy: VersionedPolicy  },

    AddMemberFromGroup { group_id: AccountId, member_id: AccountId, rol_name: String, },
    
    RemoveMemberFromGroup { group_id: AccountId, member_id: AccountId, rol_name: String, },
    
    FunctionCall {
        receiver_id: AccountId,
        actions: Vec<ActionCall>,
    },
    
    Transfer {
        token_id: Option<String>,
        receiver_id: AccountId,
        amount: U128,
        msg: Option<String>,
    },

    ChangePolicyAddOrUpdateRole { rol_name: String, role: RolePermission },

    ChangePolicyRemoveRole { rol_name: String },

    ChangePolicyUpdateVotePolicy { proposal_kind: String, vote_policy: VotePolicy },

    ChangePolicyUpdateParameters { proposal_kind: String, parameters: PolicyParameters },

    Voting,
}

impl ProposalKind {
  pub fn to_label(&self) -> &str {
    match self {
      ProposalKind::ChangePolicy { .. } => "ChangePolicy",
      ProposalKind::AddMemberFromGroup { .. } => "AddMemberFromGroup",
      ProposalKind::RemoveMemberFromGroup { .. } => "RemoveMemberFromGroup",
      ProposalKind::FunctionCall { .. } => "FunctionCall",
      ProposalKind::Transfer { .. } => "Transfer",
      ProposalKind::ChangePolicyAddOrUpdateRole { .. } => "ChangePolicyAddOrUpdateRole",
      ProposalKind::ChangePolicyRemoveRole { .. } => "ChangePolicyRemoveRole",
      ProposalKind::ChangePolicyUpdateVotePolicy { .. } => "ChangePolicyUpdateVotePolicy",
      ProposalKind::ChangePolicyUpdateParameters { .. } => "ChangePolicyUpdateParameters",
      ProposalKind::Voting => "Voting",
    }
  }
}

pub fn proposal_kind_is_exists(label: String) -> bool {
    let result: Vec<String> = vec![
      "AddMemberGroup".to_string(),
      "RemoveMemberFromGroup".to_string(),
      "FunctionCall".to_string(),
      "Transfer".to_string()
    ];
    
    result.contains(&label)
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposal {
  pub title: String,
  pub description: String,
  pub proposer: AccountId,
  pub kind: ProposalKind,
  pub group: Option<AccountId>,
  pub submission_time: U64,
  pub vote_counts: UnorderedMap<Action, u128>,
  pub votes: UnorderedMap<AccountId, Action>,
  pub status: ProposalStatus,
  pub approval_date: Option<U64>,
  pub link: String,
  pub admin_appoved: bool,
}

impl Proposal {
  pub fn update_votes(
      &mut self,
      account_id: AccountId,
      action: &Action,
  ) {
    assert!(
      self.votes.get(&account_id).is_none(),
      "ERR_ALREADY_VOTED"
    );
      let mut votes: u128 = 0;
      let vote_counts = self.vote_counts.get(action);
      if vote_counts.is_some() {
        votes = vote_counts.unwrap() + 1u128;
      }

      self.votes.insert(&account_id.clone(), action);
      self.vote_counts.insert(action, &votes);
  }
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalImput {
  title: String,
  description: String,
  proponent: AccountId,
  group: Option<AccountId>,
  kind: ProposalKind,
  link: String,
}

impl Contract {
  pub(crate) fn internal_payout(
    &mut self,
    token_id: Option<String>,
    receiver_id: &AccountId,
    amount: Balance,
    memo: String,
    msg: Option<String>,
  ) -> PromiseOrValue<()> {
      if token_id.is_none() {
          Promise::new(receiver_id.clone()).transfer(amount).into()
      } else {
          if let Some(msg) = msg {
              ext_fungible_token::ft_transfer_call(
                  receiver_id.clone(),
                  U128(amount),
                  Some(memo),
                  msg,
                  AccountId::new_unchecked(token_id.as_ref().unwrap().clone()),
                  ONE_YOCTO_NEAR,
                  GAS_FOR_FT_TRANSFER,
              )
          } else {
              ext_fungible_token::ft_transfer(
                  receiver_id.clone(),
                  U128(amount),
                  Some(memo),
                  AccountId::new_unchecked(token_id.as_ref().unwrap().clone()),
                  ONE_YOCTO_NEAR,
                  GAS_FOR_FT_TRANSFER,
              )
          }
          .into()
      }
  }

  fn internal_return_bonds(&mut self, policy: &Policy, proposal: &Proposal) -> Promise {
    let proposal_bond = policy.get_proposal_bond(&proposal.kind.to_label().to_string());
    self.locked_amount -= proposal_bond.0; 
    Promise::new(proposal.proposer.clone()).transfer(proposal_bond.0)
  }

  /// Executes given proposal and updates the contract's state.
  fn internal_execute_proposal(
      &mut self,
      policy: Policy,
      proposal: &Proposal,
      proposal_id: u128,
  ) -> PromiseOrValue<()> {
    let result = match &proposal.kind {
      ProposalKind::ChangePolicy { policy } => {
          self.policy.set(policy);

          PromiseOrValue::Value(())
      },
      ProposalKind::AddMemberFromGroup { group_id, member_id, rol_name } => {
        let group = self.groups.get(&group_id).expect("ERR_GROUP_NOT_FOUND");
        let mut new_policy = group.to_policy().clone();
        new_policy.add_member_to_role(rol_name.to_string(), member_id.clone());
        
        self.groups.insert(&group_id, &VersionedPolicy::Current(new_policy));

        PromiseOrValue::Value(())
      },
      ProposalKind::RemoveMemberFromGroup {group_id, member_id, rol_name } => {
        let mut group = self.groups.get(&group_id).expect("ERR_GROUP_NOT_FOUND");
        let mut new_policy = group.to_policy().clone();
        new_policy.remove_member_from_role(rol_name.to_string(), member_id.clone());

        self.groups.insert(&group_id, &VersionedPolicy::Current(new_policy));

        PromiseOrValue::Value(())
      },
      ProposalKind::FunctionCall {
          receiver_id,
          actions,
      } => {
          let mut promise = Promise::new(receiver_id.clone().into());
          for action in actions {
              promise = promise.function_call(
                  action.method_name.clone().into(),
                  action.args.clone().into(),
                  action.deposit.0,
                  Gas(action.gas.0),
              )
          }
          promise.into()
      },
      ProposalKind::Transfer {
        token_id,
        receiver_id,
        amount,
        msg,
      } => self.internal_payout(
          token_id.clone(),
          &receiver_id,
          amount.0,
          proposal.description.clone(),
          msg.clone(),
      ),
      ProposalKind::ChangePolicyAddOrUpdateRole { rol_name, role } => {
          let mut new_policy = policy.clone();
          new_policy.add_or_update_role(rol_name.to_string(), role.clone());
          self.policy.set(&VersionedPolicy::Current(new_policy));

          PromiseOrValue::Value(())
      }
      ProposalKind::ChangePolicyRemoveRole { rol_name } => {
          let mut new_policy = policy.clone();
          new_policy.remove_role(rol_name.to_string());
          self.policy.set(&VersionedPolicy::Current(new_policy));

          PromiseOrValue::Value(())
      }
      ProposalKind::ChangePolicyUpdateVotePolicy { proposal_kind, vote_policy } => {
          let mut new_policy = policy.clone();
          new_policy.add_or_update_vote_policy(proposal_kind.to_string(), vote_policy.clone());
          self.policy.set(&VersionedPolicy::Current(new_policy));

          PromiseOrValue::Value(())
      }
      ProposalKind::ChangePolicyUpdateParameters { proposal_kind, parameters } => {
          let mut new_policy = policy.clone();
          new_policy.update_parameters(proposal_kind.to_string(), parameters.clone());
          self.policy.set(&VersionedPolicy::Current(new_policy));
          PromiseOrValue::Value(())
      }
      _=> PromiseOrValue::Value(())
    };
    match result {
        PromiseOrValue::Promise(promise) => promise
            .then(ext_self::on_proposal_callback(
                proposal_id,
                env::current_account_id(),
                0,
                GAS_FOR_FT_TRANSFER,
            ))
            .into(),
        PromiseOrValue::Value(()) => near_sdk::PromiseOrValue::Promise(self.internal_return_bonds(&policy, &proposal)),
    }
  }

  pub(crate) fn internal_callback_proposal_success(
      &mut self,
      proposal: &mut Proposal,
  ) -> PromiseOrValue<()> {
      let policy = self.policy.get().unwrap().to_policy();
      
      proposal.status = ProposalStatus::Approved;
      near_sdk::PromiseOrValue::Promise(self.internal_return_bonds(&policy, &proposal))
  }

  pub(crate) fn internal_callback_proposal_fail(
      &mut self,
      proposal: &mut Proposal,
  ) -> PromiseOrValue<()> {
      proposal.status = ProposalStatus::Failed;
      PromiseOrValue::Value(())
  }

  /// Process rejecting proposal.
  fn internal_reject_proposal(
      &mut self,
      policy: &Policy,
      proposal: &Proposal,
      return_bonds: bool,
  ) {
      if return_bonds {
          // Return bond to the proposer.
          self.internal_return_bonds(policy, proposal);
      }
  }

}

#[near_bindgen]
impl Contract {

  // agregar propuesta
  #[payable]
  pub fn set_proposal(&mut self, data: ProposalImput) {
    let policy = self.policy.get().unwrap().to_policy();
    let proposal_bond = policy.proposal_bond.get(&data.kind.to_label().to_string()).expect("ERR_PROPOSAL_BOND_NOT_FOUND");
    let attached_deposit = env::attached_deposit();
    
    assert!(
      attached_deposit >= proposal_bond.0,
        "ERR_MIN_BOND"
    );

    ext_contract_nft::is_member(
      env::predecessor_account_id(),
      AccountId::new_unchecked(CONTRACT_NFT.to_string()),
      0,
      BASE_GAS,
    ).then(ext_self::on_set_proposal(
        data
        , attached_deposit
        , env::current_account_id()
        , 0
        , Gas(200_000_000_000_000)
    ));

  }


  #[private]
  pub fn on_set_proposal(&mut self, data: ProposalImput, attached_deposit: Balance) -> u128 {
    assert_eq!(env::promise_results_count(), 1, "ERR_UNEXPECTED_CALLBACK_PROMISES");
    
    match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Successful(val) => {
            if let Ok(is_allowlisted) = near_sdk::serde_json::from_slice::<U128>(&val) {
              self._internal_callback_set_proposal(data, attached_deposit, is_allowlisted)
          } else {
              env::panic_str("ERR_WRONG_VAL_RECEIVED")
          }
        },
        PromiseResult::Failed => env::panic_str("ERR_CALL_FAILED"),
    }
    
    
  }

  fn _internal_callback_set_proposal(&mut self, data: ProposalImput, attached_deposit: Balance, members: U128) -> u128 {
    let policy = self.policy.get().unwrap().to_policy();
    let id = self.last_proposal_id;
    let user_creation: AccountId = env::signer_account_id();
    let submission_time: u64 = env::block_timestamp();

    let proposal: &Proposal = &Proposal {
      title: data.title.clone(),
      description: data.description.clone(),
      proposer: user_creation.clone(),
      submission_time: U64::from(submission_time),
      kind: data.kind,
      group: data.group.clone(),
      vote_counts: UnorderedMap::new(
        StorageKey::KeyProposalsByVoteCounts {
          proposals_id: id,
        }
        .try_to_vec()
        .unwrap(),
      ),
      votes: UnorderedMap::new(
        StorageKey::KeyProposalsByVotes {
          proposals_id: id,
        }
        .try_to_vec()
        .unwrap(),
      ),
      status: ProposalStatus::InProgress,
      approval_date: None,
      link: data.link.clone(),
      admin_appoved: false,
    };
    
    
    self.proposals.insert(&id, proposal);

    self.last_proposal_id += 1;
    self.locked_amount += attached_deposit;

    env::log_str(
      &json!({
        "id": id.to_string(),
        "title": data.title,
        "proposal_type": proposal.kind.to_label().to_string(),
        "kind": json!(proposal.kind).to_string(),
        "description": data.description,
        "proposer": data.proponent,
        "group": data.group,
        "submission_time": submission_time.to_string(),
        "status": ProposalStatus::InProgress,
        "creation_date": env::block_timestamp().to_string(),
        "user_creation": user_creation,
        "link": data.link
      }).to_string(),
    );

    id
  }


  // agregar propuesta
  #[payable]
  pub fn update_proposal(&mut self, id: u128, action: Action, memo: Option<String>) {
    ext_contract_nft::is_member(
      env::predecessor_account_id(),
      AccountId::new_unchecked(CONTRACT_NFT.to_string()),
      0,
      BASE_GAS,
    ).then(ext_self::on_update_proposal(
        id
        , action
        , memo
        , env::current_account_id()
        , 10000000000000000000000000
        , Gas(20_000_000_000_000)
    ));
  }



  #[private] 
  #[payable]
  pub fn on_update_proposal(&mut self, id: u128, action: Action, memo: Option<String>) -> Option<u128> {
    assert_eq!(env::promise_results_count(), 1, "ERR_UNEXPECTED_CALLBACK_PROMISES");
    match env::promise_result(0) {
        PromiseResult::NotReady => {unreachable!(); None},
        PromiseResult::Successful(val) => {
            if let Ok(is_allowlisted) = near_sdk::serde_json::from_slice::<U128>(&val) {
              Some(self._internal_callback_update_proposal(id, &action, memo, is_allowlisted.0))
          } else {
              env::panic_str("ERR_WRONG_VAL_RECEIVED");
              None
          }
        },
        PromiseResult::Failed => {env::panic_str("ERR_CALL_FAILED"); None },
    }
  }

  
  fn _internal_callback_update_proposal(&mut self, id: u128, action: &Action, memo: Option<String>, members: u128) -> u128 {
    let initial_storage_usage = env::storage_usage();

    let mut proposal: Proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
    let mut typeAction = "";
    
    let sender_id = env::signer_account_id();
    let policy = self.policy.get().unwrap().to_policy();
    // Check permissions for the given action.
    let allowed =
        policy.check_permission(sender_id.clone(), &proposal.kind, action);
    assert!(allowed, "ERR_PERMISSION_DENIED");
    
    
    let update = match action {
        Action::AddProposal => env::panic_str("ERR_WRONG_ACTION"),
        Action::RemoveProposal => {
            self.proposals.remove(&id);
            false
        }
        Action::VoteApprove | Action::VoteReject | Action::VoteRemove => {
            assert!(
                matches!(proposal.status, ProposalStatus::InProgress),
                "ERR_PROPOSAL_NOT_READY_FOR_VOTE"
            );

            if self.administrators.contains(&sender_id.clone()) {
              if action.to_label() == "VoteApprove" {
                proposal.admin_appoved = true;
              }
            }

            proposal.update_votes(sender_id.clone(), &action);

            typeAction = "vote";

            self.proposals.insert(&id, &proposal);

            proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
            
            // Updates proposal status with new votes using the policy.
            proposal.status = policy.proposal_status(&proposal, members, proposal.admin_appoved.clone());

            if proposal.status == ProposalStatus::Approved  {
                self.internal_execute_proposal(policy, &proposal, id);
                true
            } else if proposal.status == ProposalStatus::Removed {
                self.internal_reject_proposal(&policy, &proposal, false);
                self.proposals.remove(&id);
                false
            } else if proposal.status == ProposalStatus::Rejected {
                self.internal_reject_proposal(&policy, &proposal, true);
                true
            } else {
                // Still in progress or expired.
                true
            }
        }
        // There are two cases when proposal must be finalized manually: expired or failed.
        // In case of failed, we just recompute the status and if it still approved, we re-execute the proposal.
        // In case of expired, we reject the proposal and return the bond.
        // Corner cases:
        //  - if proposal expired during the failed state - it will be marked as expired.
        //  - if the number of votes in the group has changed (new members has been added) -
        //      the proposal can loose it's approved state. In this case new proposal needs to be made, this one can only expire.
        Action::Finalize => {
            proposal.status = policy.proposal_status(&proposal, members, proposal.admin_appoved.clone());
            match proposal.status {
                ProposalStatus::Approved => {
                    self.internal_execute_proposal(policy, &proposal, id);
                }
                ProposalStatus::Expired => {
                    self.internal_reject_proposal(&policy, &proposal, true);
                }
                _ => {
                    env::panic_str("ERR_PROPOSAL_NOT_EXPIRED_OR_FAILED");
                }
            }
            true
        }
    };
    if update {
        self.proposals.insert(&id, &proposal);
    }

    env::log_str(
      &json!({
        "id": id.to_string(),
        "type": typeAction,
        "action": action,
        "status": proposal.status,
        "memo": memo.clone(),
        "sender_id": sender_id.to_string(),
        "admin_appoved": proposal.admin_appoved,
      }).to_string(),
    );

    refund_deposit(env::storage_usage() - initial_storage_usage, 0)
  }


  #[private]
  pub fn on_proposal_callback(&mut self, proposal_id: u128) -> PromiseOrValue<()> {
    let mut proposal: Proposal = self
        .proposals
        .get(&proposal_id)
        .expect("ERR_NO_PROPOSAL");

    assert_eq!(
        env::promise_results_count(),
        1,
        "ERR_UNEXPECTED_CALLBACK_PROMISES"
    );
    let result = match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Successful(_) => {
          let type_proposal = proposal.kind.clone().to_label().to_string();
          let (token_id, sender_id, amount): (Option<String>, Option<String>, Option<String>) = match proposal.kind.clone() {
            ProposalKind::Transfer { token_id, receiver_id, amount, msg } => (Some(token_id.unwrap_or("NEAR".to_string())), Some(receiver_id.to_string()), Some(amount.0.to_string())),
            _=> (None, None, None)
          };

          env::log_str(
            &json!({
              "id": proposal_id.to_string(),
              "type": type_proposal,
              "status": proposal.status,
              "sender_id": sender_id,
              "amount": amount,
              "token_id": token_id,
            }).to_string()
          );
          self.internal_callback_proposal_success(&mut proposal)
        },
        PromiseResult::Failed => self.internal_callback_proposal_fail(&mut proposal),
    };
    self.proposals.insert(&proposal_id, &proposal);

    result
  }
}


fn refund_deposit(storage_used: u64, extra_spend: Balance) -> u128 {
  let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
  
  required_cost
}

/*
  {
  pub title: String,
  pub description: String,
  pub proposer: AccountId,
  pub kind: ProposalKind,
  pub group: Option<AccountId>,
  pub submission_time: U64,
  pub vote_counts: UnorderedMap<Action, u128>,
  pub votes: UnorderedMap<AccountId, Action>,
  pub status: ProposalStatus,
  pub approval_date: Option<U64>,
  pub link: String,
}
*/