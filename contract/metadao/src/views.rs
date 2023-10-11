use crate::*;

/*// This is format of output via JSON for the proposal.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalOutput {
    /// Id of the proposal.
    pub id: u64,
    #[serde(flatten)]
    pub proposal: Proposal,
}*/


#[near_bindgen]
impl Contract {
    pub fn get_staking_contract(self) -> AccountId {
        self.owner_id
    }

    pub fn get_locked_storage_amount(&self) -> U128 {
        let locked_storage_amount = env::storage_byte_cost() * (env::storage_usage() as u128);
        U128(locked_storage_amount)
    }

    pub fn get_available_amount(&self) -> U128 {
        U128(env::account_balance() - self.get_locked_storage_amount().0 - self.locked_amount)
    }

    pub fn delegation_total_supply(&self) -> U128 {
        U128(self.total_delegation_amount)
    }

    pub fn delegation_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.delegations.get(&account_id).unwrap_or_default())
    }

    pub fn delegation_balance_ratio(&self, account_id: AccountId) -> (U128, U128) {
        (
            self.delegation_balance_of(account_id),
            self.delegation_total_supply(),
        )
    }

    /*pub fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<ProposalOutput> {
        (from_index..min(self.last_proposal_id, from_index + limit))
            .filter_map(|id| {
                self.proposals.get(&id).map(|proposal| ProposalOutput {
                    id,
                    proposal: proposal.into(),
                })
            })
            .collect()
    }

    pub fn get_proposal(&self, id: u64) -> ProposalOutput {
        let proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
        ProposalOutput {
            id,
            proposal: proposal.into(),
        }
    }*/

}