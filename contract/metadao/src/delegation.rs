use crate::*;


#[near_bindgen]
impl Contract {
    /*#[payable]
    pub fn register_delegation(&mut self, account_id: &AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "ERR_INVALID_CALLER"
        );
        assert_eq!(env::attached_deposit(), 16 * env::storage_byte_cost());
        self.delegations.insert(account_id, &0);
    }*/

    #[payable]
    pub fn delegate(&mut self, account_id: &AccountId, amount: U128) -> (U128, U128, U128) {
        /*assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "ERR_INVALID_CALLER"
        );*/

        let delegacion = self.delegations.get(account_id);

        let mut prev_amount = 0;
        
        if delegacion.is_some() {
            prev_amount = delegacion.unwrap(); /*self
            .delegations
            .get(account_id)
            .expect("ERR_NOT_REGISTERED");*/
        }
        

        
        let new_amount = prev_amount + amount.0;
        
        self.delegations.insert(account_id, &new_amount);
        
        self.total_delegation_amount += amount.0;
        
        env::log_str(
            &json!({
                "prev_amount": prev_amount.to_string(),
                "new_amount": new_amount.to_string(),
                "delegate_amount": amount.0.to_string(),
                "delegacion_total": self.delegation_total_supply().0.to_string(),
                "delegator": account_id.to_string(),
            }).to_string(),
        );
        
        (
            U128(prev_amount),
            U128(new_amount),
            self.delegation_total_supply(),
        )

    }

    // Removes given amount from given account's delegations.
    // Returns previous, new amount of this account and total delegated amount.
    /* pub fn undelegate(&mut self, account_id: &AccountId, amount: U128) -> (U128, U128, U128) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "ERR_INVALID_CALLER"
        );
        let prev_amount = self.delegations.get(account_id).unwrap_or_default();
        assert!(prev_amount >= amount.0, "ERR_INVALID_STAKING_CONTRACT");
        let new_amount = prev_amount - amount.0;
        self.delegations.insert(account_id, &new_amount);
        self.total_delegation_amount -= amount.0;
        (
            U128(prev_amount),
            U128(new_amount),
            self.delegation_total_supply(),
        )
    } */
}