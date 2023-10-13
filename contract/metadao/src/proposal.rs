use crate::*;
use near_contract_standards::fungible_token::core_impl::ext_fungible_token;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
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
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
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
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalKind {

    ChangePolicy { policy: VersionedPolicy  },

    AddMemberFromGroup { group_id: AccountId, member_id: AccountId },
    
    RemoveMemberFromGroup { group_id: AccountId, member_id: AccountId },
    
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
      ProposalKind::AddMemberFromGroup { group_id, member_id } => {
        let mut group = self.groups.get(&group_id).expect("ERR_GROUP_NOT_FOUND");
        group.insert(&member_id);
        self.groups.insert(&group_id, &group);

        PromiseOrValue::Value(())
      },
      ProposalKind::RemoveMemberFromGroup {group_id, member_id } => {
        let mut group = self.groups.get(&group_id).expect("ERR_GROUP_NOT_FOUND");
        group.remove(&member_id);
        self.groups.insert(&group_id, &group);

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
    assert!(
        env::attached_deposit() >= proposal_bond.0,
        "ERR_MIN_BOND"
    );

    ext_contract_nft::is_member(
      env::predecessor_account_id(),
      AccountId::new_unchecked(CONTRACT_NFT.to_string()),
      0,
      BASE_GAS,
    ).then(ext_self::on_set_proposal(
        data
        , env::current_account_id()
        , 0
        , Gas(200_000_000_000_000)
    ));

  }


  #[private]
  pub fn on_set_proposal(&mut self, data: ProposalImput) -> u128 {
    assert_eq!(env::promise_results_count(), 1, "ERR_UNEXPECTED_CALLBACK_PROMISES");
    
    match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Successful(val) => {
            if let Ok(is_allowlisted) = near_sdk::serde_json::from_slice::<U128>(&val) {
              self._internal_callback_set_proposal(data, is_allowlisted)
          } else {
              env::panic_str("ERR_WRONG_VAL_RECEIVED")
          }
        },
        PromiseResult::Failed => env::panic_str("ERR_CALL_FAILED"),
    }
    
    
  }

  fn _internal_callback_set_proposal(&mut self, data: ProposalImput, members: U128) -> u128 {
    let policy = self.policy.get().unwrap().to_policy();
    let id = self.last_proposal_id;
    let user_creation: AccountId = env::predecessor_account_id();
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
    };
    
    
    self.proposals.insert(&id, proposal);

    self.last_proposal_id += 1;
    self.locked_amount += env::attached_deposit();

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
  /*#[payable]
  pub fn set_proposal(&mut self, data: ProposalImput) -> u128 {
    let policy = self.policy.get().unwrap().to_policy();
    let proposal_bond = policy.proposal_bond.get(&data.kind.to_label().to_string()).expect("ERR_PROPOSAL_BOND_NOT_FOUND");
    assert!(
        env::attached_deposit() >= proposal_bond.0,
        "ERR_MIN_BOND"
    );
    
    let id = self.last_proposal_id;
    let user_creation: AccountId = env::predecessor_account_id();
    let submission_time: u64 = env::block_timestamp();

    let proposal: &Proposal = &Proposal {
      title: data.title.clone(),
      description: data.description.clone(),
      proposer: data.proponent.clone(),
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
    };
    
    
    self.proposals.insert(&id, proposal);

    self.last_proposal_id += 1;
    self.locked_amount += env::attached_deposit();

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
  }*/


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
        , 0
        , Gas(200_000_000_000_000)
    ));

  }


  #[private]
  pub fn on_update_proposal(&mut self, id: u128, action: Action, memo: Option<String>) {
    assert_eq!(env::promise_results_count(), 1, "ERR_UNEXPECTED_CALLBACK_PROMISES");
    match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Successful(val) => {
            if let Ok(is_allowlisted) = near_sdk::serde_json::from_slice::<U128>(&val) {
              self._internal_callback_update_proposal(id, &action, memo, is_allowlisted.0);
          } else {
              env::panic_str("ERR_WRONG_VAL_RECEIVED")
          }
        },
        PromiseResult::Failed => env::panic_str("ERR_CALL_FAILED"),
    }
  }

  
  fn _internal_callback_update_proposal(&mut self, id: u128, action: &Action, memo: Option<String>, members: u128)  {
    let mut proposal: Proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
    let mut typeAction = "";
    
    let sender_id = env::predecessor_account_id();
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
            
            proposal.update_votes(sender_id.clone(), &action);

            typeAction = "vote";
            
            // Updates proposal status with new votes using the policy.
            proposal.status = policy.proposal_status(&proposal);

            if proposal.status == ProposalStatus::Approved {
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
            /*proposal.status = policy.proposal_status(
                &proposal,
                policy.roles.iter().map(|r| r.name.clone()).collect(),
                self.total_delegation_amount,
            );
            match proposal.status {
                ProposalStatus::Approved => {
                    self.internal_execute_proposal(&policy, &proposal, id);
                }
                ProposalStatus::Expired => {
                    self.internal_reject_proposal(&policy, &proposal, true);
                }
                _ => {
                    env::panic_str("ERR_PROPOSAL_NOT_EXPIRED_OR_FAILED");
                }
            }*/
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
        "sender_id": sender_id.to_string()
      }).to_string(),
    );
  }

  /*pub fn update_proposal(&mut self, id: u128, action: &Action, memo: Option<String>) {
    let mut proposal: Proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
    let mut typeAction = "";
    
    let sender_id = env::predecessor_account_id();
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
            
            proposal.update_votes(sender_id.clone(), &action);

            typeAction = "vote";
            
            // Updates proposal status with new votes using the policy.
            proposal.status = policy.proposal_status(&proposal);

            if proposal.status == ProposalStatus::Approved {
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
            /*proposal.status = policy.proposal_status(
                &proposal,
                policy.roles.iter().map(|r| r.name.clone()).collect(),
                self.total_delegation_amount,
            );
            match proposal.status {
                ProposalStatus::Approved => {
                    self.internal_execute_proposal(&policy, &proposal, id);
                }
                ProposalStatus::Expired => {
                    self.internal_reject_proposal(&policy, &proposal, true);
                }
                _ => {
                    env::panic_str("ERR_PROPOSAL_NOT_EXPIRED_OR_FAILED");
                }
            }*/
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
        "sender_id": sender_id.to_string()
      }).to_string(),
    );
  }*/

  #[private]
  pub fn on_proposal_callback(&mut self, proposal_id: u128) -> PromiseOrValue<()> {
    let mut proposal: Proposal = self
        .proposals
        .get(&proposal_id)
        .expect("ERR_NO_PROPOSAL")
        .into();
    assert_eq!(
        env::promise_results_count(),
        1,
        "ERR_UNEXPECTED_CALLBACK_PROMISES"
    );
    let result = match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Successful(_) => self.internal_callback_proposal_success(&mut proposal),
        PromiseResult::Failed => self.internal_callback_proposal_fail(&mut proposal),
    };
    self.proposals
        .insert(&proposal_id, &proposal);
    result
  }
}