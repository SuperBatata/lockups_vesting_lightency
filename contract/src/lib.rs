use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};


// contract for a lockup contract to be used with a specific token.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]


pub struct LockupContract {
    // the token contract address
    pub token_contract_address: String,
    // the amount of tokens to be locked
    pub amount: u128,
    // the amount of time to lock the tokens for
    pub duration: u128,
    // the amount of time that has passed since the lockup was created
    pub time_passed: u128,
    // the amount of tokens that have been locked
    pub locked_amount: u128,
    // the amount of tokens that have been unlocked
    pub unlocked_amount: u128,
    // the amount of tokens that have been locked and unlocked
    pub total_amount: u128,
    // the amount of tokens that have been locked and unlocked
    pub total_amount_locked: u128,

}


#[near_bindgen]

impl LockupContract{
    pub fn new (token_contract_address: String, amount: u128, duration: u128) -> Self {
        Self {
            token_contract_address,
            amount,
            duration,
            time_passed: 0,
            locked_amount: 0,
            unlocked_amount: 0,
            total_amount: 0,
            total_amount_locked: 0,
        }
    }

    pub fn get_token_contract_address(&self) -> String {
        self.token_contract_address.clone()
    }
    pub fn get_amount(&self) -> u128 {
        self.amount
    }
}



