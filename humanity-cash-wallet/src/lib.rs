use hex;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Promise, PromiseResult};
use sha3::{Digest, Keccak256};
use std::time::SystemTime;

#[ext_contract(ext_itoken)]
trait IToken {
    // change methods
    fn ft_transfer(&mut self, receiver_id: String, amount: String, memo: Option<String>);

    // view methods
    fn ft_balance_of(&self, account_id: String) -> String;
}

#[ext_contract(ext_iwallet)]
trait IWallet {
    // view methods
    fn available_balance_callback(&self) -> u128;
}

fn keccak256(text: String) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Wallet {
    erc20_token: AccountId,
    created_block: u64,
    user_id: AccountId,
    controller_role: String,
}

#[near_bindgen]
impl Wallet {
    pub fn get_version_number() -> (u8, u8, u8, u8) {
        (1, 2, 0, 0)
    }

    pub fn initialize(
        &mut self,
        erc20_token: AccountId,
        controller: AccountId,
        user_id: AccountId,
    ) {
        self.erc20_token = erc20_token;
        self.created_block =
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        self.user_id = user_id;
        self.controller_role = keccak256(String::from("CONTROLLER_ROLE"));
    }

    pub fn available_balance_callback(&self) -> u128 {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => 0,
            PromiseResult::Successful(result) => {
                let balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap();
                balance.0
            }
        }
    }

    pub fn available_balance(&self) -> Promise {
        ext_itoken::ft_balance_of(
            env::current_account_id().to_string(), // ext_itoken takes an account_id as a parameter
            &self.erc20_token.to_string(),         // contract account id
            0,                                     // yocto NEAR to attach
            5_000_000_000_000,                     // gas to attach
        )
        .then(ext_iwallet::available_balance_callback(
            &env::current_account_id(), // this contract's account id
            0,                          // yocto NEAR to attach to the callback
            5_000_000_000_000,          // gas to attach to the callback
        ))
    }
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
