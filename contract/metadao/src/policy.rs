use crate::*;


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct RolePermission {
    pub name: String,
    pub users: HashSet<AccountId>,
    pub permissions: HashSet<String>,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct VotePolicy {
    pub quorum: U128,
    pub threshold: U128,
    pub percentage: f64, 
    pub is_percentage: bool,
}

/*impl Default for VotePolicy {
    fn default() -> Self {
        VotePolicy {
            quorum: U128(0u128),
            threshold: U128(0u128),
            percentage: 50.0,
            is_percentage: true,
        }
    }
}*/


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    
    pub roles: HashMap<String, RolePermission>,
    
    pub vote_policy: HashMap<String, VotePolicy>,
    
    pub proposal_bond: HashMap<String, U128>,
    
    pub proposal_period: HashMap<String, U64>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum VersionedPolicy {
    /// Default policy with given accounts as council.
    Default(Vec<AccountId>),
    Current(Policy),
}

pub fn default_policy(council: Vec<AccountId>) -> Policy {
  
  // let mut permissions = HashSet::new();
  // permissions.insert("*:*".to_string());

  let mut roles = HashMap::new();
  roles.insert("council".to_string(), RolePermission {
    name: "council".to_string(),
    users:  council.into_iter().collect::<HashSet<AccountId>>().into(),
    permissions: ["*:*".to_string()].into(),
  });

  let proposal_kind: Vec<&str> = vec![
    "ChangePolicy",
    "AddMemberFromGroup",
    "RemoveMemberFromGroup",
    "FunctionCall",
    "Transfer",
    "ChangePolicyAddOrUpdateRole",
    "ChangePolicyRemoveRole",
    "ChangePolicyUpdateVotePolicy",
    "ChangePolicyUpdateParameters",
    "Voting"
  ];

  let mut vote_policy = HashMap::new();
  let mut proposal_bond = HashMap::new();
  let mut proposal_period = HashMap::new();

  let vote_policy_default: VotePolicy = VotePolicy {
    quorum: U128(5u128),
    threshold: U128(3u128),
    percentage: 50.0, 
    is_percentage: true,
  };
  let proposal_bond_default: U128 = U128(10u128.pow(22));
  let proposal_period_default: U64 = U64(1_000_000_000 * 60 * 60 * 24 * 7);
  
  for kind in proposal_kind.iter() {
    vote_policy.insert(kind.to_string(), vote_policy_default.clone());
    proposal_bond.insert(kind.to_string(), proposal_bond_default);
    proposal_period.insert(kind.to_string(), proposal_period_default);
  }

  env::log_str(
    &json!({
      "roles": json!(roles).to_string(),
      "vote_policy": json!(vote_policy).to_string(),
      "proposal_bond": json!(proposal_bond).to_string(),
      "proposal_period": json!(proposal_period).to_string(),
    }).to_string(),
  );
  
  Policy {
      roles: roles,
      vote_policy: vote_policy,
      proposal_bond: proposal_bond,
      proposal_period: proposal_period,
  }

}

impl VersionedPolicy {
  /// Upgrades either version of policy into the latest.
  pub fn upgrade(self) -> Self {
      match self {
          VersionedPolicy::Default(accounts) => {
              VersionedPolicy::Current(default_policy(accounts))
          }
          VersionedPolicy::Current(policy) => VersionedPolicy::Current(policy),
      }
  }

  /// Return recent version of policy.
  pub fn to_policy(self) -> Policy {
      match self {
          VersionedPolicy::Current(policy) => policy,
          _ => unimplemented!(),
      }
  }

  pub fn to_policy_mut(&mut self) -> &mut Policy {
      match self {
          VersionedPolicy::Current(policy) => policy,
          _ => unimplemented!(),
      }
  }
}


impl Policy {
  pub fn add_or_update_role(&mut self, rol_name: String, role: RolePermission) {
    self.roles.insert(rol_name, role);
  }

  pub fn remove_role(&mut self, role_name: String) {
    self.roles.remove(&role_name);
  }

  pub fn add_or_update_vote_policy(&mut self, proposal_kind: String, vote_policy: VotePolicy) {
    if proposal_kind_is_exists(proposal_kind.clone()) { 
      self.vote_policy.insert(proposal_kind, vote_policy);
    }
  }

  pub fn update_parameters(&mut self, proposal_kind: String, parameters: PolicyParameters) {
    if proposal_kind_is_exists(proposal_kind.clone()) { 
      if parameters.proposal_bond.is_some() {
        self.proposal_bond.insert(proposal_kind.clone(), parameters.proposal_bond.unwrap());
      }
      
      if parameters.proposal_period.is_some() {
        self.proposal_period.insert(proposal_kind, parameters.proposal_period.unwrap());
      }
    }
  }

  pub fn add_member_to_role(&mut self, rol_name: String, member_id: AccountId) {
      let mut role = self.roles.get(&rol_name.clone()).expect(&format!("ERR_ROLE_NOT_FOUND:{}", rol_name).to_string()).clone();

      role.users.insert(member_id);
      self.roles.insert(rol_name, role);
  }

  pub fn remove_member_from_role(&mut self, rol_name: String, member_id: AccountId) {
    let mut role = self.roles.get(&rol_name.clone()).expect(&format!("ERR_ROLE_NOT_FOUND:{}", rol_name).to_string()).clone();

      role.users.remove(&member_id);
      self.roles.insert(rol_name, role);
  }


  fn get_user_roles(&self, user: AccountId) -> HashMap<String, HashSet<String>> {
      let mut roles = HashMap::default();
      
      for (k, role) in self.roles.iter() {
          if role.users.contains(&user) {
              roles.insert(k.clone(), role.permissions.clone());
          }
      }
      roles
  }

  pub fn get_proposal_bond(&self, proposal_kind: &String) -> U128 {
    let result = *self.proposal_bond.get(proposal_kind).expect(&format!("ERR_PROPOSAL_BOND_NOT_FOUND:{}", proposal_kind).to_string());
    
    result
  }

  pub fn check_permission(
      &self,
      user: AccountId,
      proposal_kind: &ProposalKind,
      action: &Action,
  ) -> bool {
    let roles = self.get_user_roles(user);
    let mut allowed = false;
    
    if proposal_kind.to_label() == "Transfer" || proposal_kind.to_label() == "FunctionCall" {
      return true
    }

    if action.to_label() == "AddProposal" || action.to_label() == "VoteApprove" || action.to_label() == "VoteReject" 
      || action.to_label() == "VoteRemove" {
      return true
    }

    for (_k, permissions) in roles.iter() {
      let allowed_role = permissions.contains(&format!(
          "{}:{}",
          proposal_kind.to_label(),
          action.to_label()
      )) || permissions
          .contains(&format!("{}:*", proposal_kind.to_label()))
          || permissions.contains(&format!("*:{}", action.to_label()))
          || permissions.contains("*:*");
      allowed = allowed || allowed_role;
    }
    
    allowed
  }

  fn internal_get_role(&self, name: &String) -> Option<&RolePermission> {
      self.roles.get(name)
  }

  // Get proposal status for given proposal.
  // Usually is called after changing it's state.
  pub fn proposal_status(
      &self,
      proposal: &Proposal,
      members: u128,
      admin_appoved: bool,
  ) -> ProposalStatus {
      assert!(
          matches!(
              proposal.status,
              ProposalStatus::InProgress | ProposalStatus::Failed
          ),
          "ERR_PROPOSAL_NOT_IN_PROGRESS"
      );
      let proposal_period: U64 = *self.proposal_period.get(&proposal.kind.to_label().to_string()).expect("ERR_NOT_PROPOSAL_PERIOD");

      if proposal.submission_time.0 + proposal_period.0 < env::block_timestamp() {
          // Proposal expired.
          return ProposalStatus::Expired;
      };
      
      let vote_policy = self.vote_policy.get(&proposal.kind.to_label().to_string()).expect("ERR_NOT_VOTE_POLICY");

      let percentage: f64 = vote_policy.percentage;

      // Check if there is anything voted above the threshold specified by policy for given role.
      let actions: Vec<Action>  = vec![Action::VoteApprove, Action::VoteReject, Action::VoteRemove];

      for action in actions.iter() {
        let vote_counts = proposal.vote_counts.get(&action).unwrap_or(0u128);
        let percentage_vote: f64 = ((vote_counts as f64) / (members as f64)) * 100.0;
        
        match action {
          Action::VoteApprove => if percentage_vote >= percentage { 
            let mut status_return = proposal.status.clone();
            if admin_appoved {
              status_return = ProposalStatus::Approved
            }
            return status_return
          },
          Action::VoteReject => if percentage_vote >= percentage { return ProposalStatus::Rejected},
          Action::VoteRemove => if percentage_vote >= percentage { return ProposalStatus::Removed},
          _ => continue
        }
      }
      
      proposal.status.clone()
  }
}