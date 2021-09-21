use hex;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{ext_contract, near_bindgen, AccountId};
use sha3::{Digest, Keccak256};
use std::time::SystemTime;

fn keccak256(text: String) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[ext_contract(ext_iwallet)]
trait IWallet {
    fn available_balance(&self) -> U128;

    fn transfer_to(&mut self, to_wallet: AccountId, value: U128) -> bool;
    fn withdraw(&mut self, value: U128) -> bool;
    fn transfer_controller(&mut self, new_controller: AccountId);
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
