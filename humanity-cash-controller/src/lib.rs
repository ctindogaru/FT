use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{ext_contract, near_bindgen, AccountId};

#[ext_contract(ext_iwallet)]
trait IWallet {
    // change methods
    fn transfer_to(&mut self, to_wallet: AccountId, value: U128) -> bool;
    fn withdraw(&mut self, value: U128) -> bool;
    fn transfer_controller(&mut self, new_controller: AccountId);

    // view methods
    fn available_balance(&self) -> U128;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Controller {
    // SETUP CONTRACT STATE
}

#[near_bindgen]
impl Controller {
    // ADD CONTRACT METHODS HERE
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::testing_env;

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
