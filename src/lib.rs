use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::Serialize;
use serde::Deserialize;
use near_sdk::{env, near_bindgen, AccountId, Promise, assert_one_yocto, ext_contract, Gas, promise_result_as_success, require, 
                serde_json::json, BorshStorageKey, PanicOnDefault}; // json_types::U128, 
use near_sdk::json_types::U128;
use std::collections::HashMap;
//near_sdk::setup_alloc!();

use near_sdk::collections::{/*LazyOption,*/ UnorderedMap, UnorderedSet};


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposal {
  id: u128,
  title: String,
  description: String,
  proposal_type: i64,
  proponents: Vec<AccountId>,
  time_complete: i64,
  claims_available: u128,
  amount: u128,
  upvote: UnorderedSet<AccountId>,
  downvote: UnorderedSet<AccountId>,
  status: i8,
  approval_date: Option<String>,
  creation_date: String,
  user_creation: AccountId,
  sponsor: AccountId,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalRequired {
  title: String,
  description: String,
  proposal_type: i64,
  proponents: Vec<AccountId>,
  time_complete: i64,
  amount: u128,
  sponsor: AccountId,
}



//////////////////////////////////////////////////////////////////////////////////////////////////
/// Objects Definition////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
//#[serde(crate = "near_sdk::serde")]
pub struct MetaDemocracia {
    owner_id: AccountId,
    pub proposal_type: UnorderedMap<i64, String>,
    pub proposals: UnorderedMap<u128, Proposal>,
    pub administrators: UnorderedSet<AccountId>,
    pub council: UnorderedSet<AccountId>,
    
}


#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    KeyProposalType,
    KeyProposals,
    KeyAdministrators,
    Keycouncil,
    KeyProposalsByUpvote { proposals_id: u128 },
    KeyProposalsByDownvote { proposals_id: u128 },
}

/// Implementing Struct
#[near_bindgen]
impl MetaDemocracia {
  /// Initializing contract
  #[init]
  pub fn new(owner_id: AccountId) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Self {
      owner_id: owner_id,
      proposal_type: UnorderedMap::new(StorageKey::KeyProposalType),
      proposals: UnorderedMap::new(StorageKey::KeyProposals),
      administrators: UnorderedSet::new(StorageKey::KeyAdministrators),
      council: UnorderedSet::new(StorageKey::Keycouncil),
    }
  }

  //agregar administrador a la lista
  pub fn set_admin(&mut self, user_id: AccountId) {
    assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
    
    if self.administrators.contains(&user_id) {
        env::panic_str("the user is already in the list of administrators");  
    }
        
    self.administrators.insert(&user_id);
    
  }

  // eliminar administrador de la lista
  pub fn delete_admin(&mut self, user_id: AccountId) {      
    assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
    
    if !self.administrators.contains(&user_id) {
      env::panic_str("the user is not on the administrators list");  
    }
    
    self.administrators.remove(&user_id);
    
  }

  // agregar consejal a la lista
  pub fn set_council(&mut self, user_id: AccountId) {
    assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
    
    if self.council.contains(&user_id) {
      env::panic_str("the user is already in the list of council");  
    }

    self.council.insert(&user_id);

    env::log_str(
      &json!({  
        "user_id": user_id.clone()
      }).to_string(),
    );
  }

  // eliminar consejal a la lista
  pub fn delete_council(&mut self, user_id: AccountId) {      
    assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
    
    if !self.council.contains(&user_id) {
      env::panic_str("the user is not on the council list");  
    }
    self.council.remove(&user_id);
    
    env::log_str(
      &json!({  
        "user_id": user_id.clone()
      }).to_string(),
    );
  }

  // agregar type propruesta
  pub fn set_type_proposal(&mut self, name: String) {
    assert!(self.owner_id == env::signer_account_id(), "Only administrator");
    
    let index: i64 = (self.proposal_type.len() + 1).try_into().unwrap(); 

    self.proposal_type.insert(&index, &name);

    env::log_str(
      &json!({  
        "id": index,
        "name": name.clone()
      }).to_string(),
    );
  }

  // agregar propuesta
  pub fn set_proposal(&mut self, data: ProposalRequired) {

    assert!(self.proposal_type.get(&data.proposal_type).is_some(), "proposal type does not exist");

    let index: u128 = (self.proposals.len() + 1).into();
    let user_creation: AccountId = env::predecessor_account_id();

    let proposal: Proposal = Proposal {
      id: index,
      title: data.title.clone(),
      description: data.description.clone(),
      proposal_type: data.proposal_type,
      proponents: data.proponents.clone(),
      time_complete: data.time_complete,
      claims_available: 0,
      amount: data.amount,
      upvote: UnorderedSet::new(
        StorageKey::KeyProposalsByUpvote {
          proposals_id: index,
        }
        .try_to_vec()
        .unwrap(),
      ),
      downvote: UnorderedSet::new(
        StorageKey::KeyProposalsByDownvote {
          proposals_id: index,
        }
        .try_to_vec()
        .unwrap(),
      ),
      status: 1,
      approval_date: None,
      creation_date: env::block_timestamp().to_string(),
      user_creation: user_creation.clone(),
      sponsor: data.sponsor.clone(),
    };
    
    self.proposals.insert(&index, &proposal);

    env::log_str(
      &json!({
        "id": index.to_string(),
        "title": data.title,
        "description": data.description,
        "proponents": data.proponents,
        "time_complete": data.time_complete,
        "claims_available": 0,
        "amount": data.amount,
        "status": 1,
        "creation_date": env::block_timestamp().to_string(),
        "user_creation": user_creation,
        "sponsor": data.sponsor
      }).to_string(),
    );
  }


  // voto positivo
  pub fn upvote(&mut self, proposal_id: u128) {
    let council: AccountId = env::predecessor_account_id();

    assert!(self.council.contains(&council.clone()), "Only council");
    
    let mut proposal = self.proposals.get(&proposal_id).expect("proposal does not exist");
    
    proposal.upvote.insert(&council.clone());
    proposal.downvote.remove(&council.clone());
    self.proposals.insert(&proposal_id, &proposal);

    env::log_str(
      &json!({  
        "council": council
      }).to_string(),
    );
  }
  

  // voto negativo
  pub fn downvote(&mut self, proposal_id: u128) {
    let council: AccountId = env::predecessor_account_id();

    assert!(self.council.contains(&council.clone()), "Only council");
    
    let mut proposal = self.proposals.get(&proposal_id).expect("proposal does not exist");
    
    proposal.downvote.insert(&council.clone());
    proposal.upvote.remove(&council.clone());
    self.proposals.insert(&proposal_id, &proposal);

    env::log_str(
      &json!({  
        "council": council
      }).to_string(),
    );
  }

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

