use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::Serialize;
use serde::Deserialize;
use near_sdk::{env, near_bindgen, AccountId, Promise, assert_one_yocto, ext_contract, Gas, promise_result_as_success, require, 
                serde_json::json, BorshStorageKey, PanicOnDefault, Balance, PromiseOrValue, PromiseResult};
use near_sdk::json_types::{Base64VecU8, U128, U64};
//near_sdk::setup_alloc!();

use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use std::collections::{HashMap, HashSet};

pub use crate::proposal::*;
pub use crate::policy::*;
pub use crate::delegation::*;
pub use crate::views::*;

mod proposal;
mod policy;
mod delegation;
mod views;


pub const ONE_YOCTO_NEAR: Balance = 1;

pub const GAS_FOR_FT_TRANSFER: Gas = Gas(10_000_000_000_000);
pub const BASE_GAS: Gas = Gas(3_000_000_000_000);

// pub const CONTRACT_NFT: &str = "nftv3.metademocracia.testnet";
pub const CONTRACT_NFT: &str = "nftv1.metademocracia_dao.near";


#[ext_contract(ext_self)]
pub trait ExtSelf {
    /// Callback after proposal execution.
    fn on_proposal_callback(&mut self, proposal_id: u128) -> PromiseOrValue<()>;

    fn on_set_proposal(&mut self, data: ProposalImput, attached_deposit: Balance) -> u128;

    fn on_update_proposal(&mut self, id: u128, action: Action, memo: Option<String>);
}

#[ext_contract(ext_contract_nft)]
pub trait ExtContractNftt {
    fn is_member(self, account_id: AccountId) -> U128;
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Objects Definition////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
//#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    pub owner_id: AccountId,
    
    pub policy: LazyOption<VersionedPolicy>,

    pub proposals: UnorderedMap<u128, Proposal>,

    pub last_proposal_id: u128,
    
    pub administrators: UnorderedSet<AccountId>,
  
    pub groups: UnorderedMap<AccountId, VersionedPolicy/*UnorderedSet<AccountId>*/>,
  
    pub total_delegation_amount: Balance,
   
    pub delegations: UnorderedMap<AccountId, Balance>,

    pub locked_amount: Balance,
}


#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    // KeyProposalType,
    keyPolicy,
    KeyProposals,
    KeyAdministrators,
    KeyGroups,
    KeyProposalsByVoteCounts { proposals_id: u128 },
    KeyProposalsByVotes { proposals_id: u128 },
    KeyGroupsMembers { group_id: u128 },
    KeyDelegation,
}

/// Implementing Struct
#[near_bindgen]
impl Contract {
  /// Initializing contract
  #[init]
  pub fn new(owner_id: AccountId, policy: VersionedPolicy) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Self {
      owner_id: owner_id,
      // proposal_type: UnorderedMap::new(StorageKey::KeyProposalType),
      policy: LazyOption::new(StorageKey::keyPolicy, Some(&policy.upgrade())),
      proposals: UnorderedMap::new(StorageKey::KeyProposals),
      last_proposal_id: 1,
      administrators: UnorderedSet::new(StorageKey::KeyAdministrators),
      groups: UnorderedMap::new(StorageKey::KeyGroups),
      total_delegation_amount: 0,
      delegations: UnorderedMap::new(StorageKey::KeyDelegation),
      locked_amount: 0,
    }
  }

  //agregar administrador a la lista
  pub fn set_admin(&mut self, user_id: AccountId) {
    assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
    
    if self.administrators.contains(&user_id) {
        env::panic_str("the user is already in the list of administrators");  
    }
        
    self.administrators.insert(&user_id);

    env::log_str(
      &json!({
        "user_id": user_id.to_string(),
      }).to_string(),
    );
    
  }

  // eliminar administrador de la lista
  pub fn delete_admin(&mut self, user_id: AccountId) {      
    assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
    
    if !self.administrators.contains(&user_id) {
      env::panic_str("the user is not on the administrators list");  
    }
    
    self.administrators.remove(&user_id);
    
    env::log_str(
      &json!({
        "user_id": user_id.to_string(),
      }).to_string(),
    );
  }


  // crear grupo
  /*fn _proposal_create_group(&mut self, data: ProposalRequired) {
    let result = Promise::new(subaccount_id.clone())
      .create_account()
      .transfer(env::attached_deposit())
      .deploy_contract(CODE.to_vec())
      .then(ext_subcontract::new(
          env::signer_account_id(),
          env::current_account_id(),
          AccountId::new_unchecked("vault.nearp2pdex.near".to_string()),
          amount_despliegue,
          subaccount_id.clone(),
          0,
          BASE_GAS,
      ));
  }*/

}




// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    #[test]
    fn set_admin() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MetaDemocracia::default();
        let user_id: AccountId = AccountId::new_unchecked("p2p-testnet.testnet".to_string());
        
        //contract.set_admin(user_id);
        //assert_eq!(contract.set_admin().len(), 1);
    }
    
}

