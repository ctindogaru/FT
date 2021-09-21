use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::{call, to_yocto, transaction::ExecutionStatus, view, DEFAULT_GAS};

use crate::utils::{init_with_macros as init, register_user};

#[test]
fn simulate_total_supply() {
    let initial_balance = to_yocto("100");
    let (_, ft, _) = init(initial_balance);

    let total_supply: U128 = view!(ft.ft_total_supply()).unwrap_json();

    assert_eq!(initial_balance, total_supply.0);
}

#[test]
fn simulate_simple_transfer() {
    let transfer_amount = to_yocto("100");
    let initial_balance = to_yocto("100000");
    let (root, ft, alice) = init(initial_balance);

    // Transfer from root to alice.
    // Uses default gas amount, `near_sdk_sim::DEFAULT_GAS`
    call!(
        root,
        ft.ft_transfer(alice.valid_account_id(), transfer_amount.into(), None),
        deposit = 1
    )
    .assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.valid_account_id())).unwrap_json();
    let alice_balance: U128 = view!(ft.ft_balance_of(alice.valid_account_id())).unwrap_json();
    assert_eq!(initial_balance - transfer_amount, root_balance.0);
    assert_eq!(transfer_amount, alice_balance.0);
}

#[test]
fn simulate_close_account_empty_balance() {
    let initial_balance = to_yocto("100000");
    let (_root, ft, alice) = init(initial_balance);

    let outcome = call!(alice, ft.storage_unregister(None), deposit = 1);
    outcome.assert_success();
    let result: bool = outcome.unwrap_json();
    assert!(result);
}

#[test]
fn simulate_close_account_non_empty_balance() {
    let initial_balance = to_yocto("100000");
    let (root, ft, _alice) = init(initial_balance);

    let outcome = call!(root, ft.storage_unregister(None), deposit = 1);
    assert!(!outcome.is_ok(), "Should panic");
    assert!(format!("{:?}", outcome.status())
        .contains("Can't unregister the account with the positive balance without force"));

    let outcome = call!(root, ft.storage_unregister(Some(false)), deposit = 1);
    assert!(!outcome.is_ok(), "Should panic");
    assert!(format!("{:?}", outcome.status())
        .contains("Can't unregister the account with the positive balance without force"));
}

#[test]
fn simulate_close_account_force_non_empty_balance() {
    let initial_balance = to_yocto("100000");
    let (root, ft, _alice) = init(initial_balance);

    let outcome = call!(root, ft.storage_unregister(Some(true)), deposit = 1);
    assert_eq!(
        outcome.logs()[0],
        format!("Closed @{} with {}", root.valid_account_id(), initial_balance)
    );
    outcome.assert_success();
    let result: bool = outcome.unwrap_json();
    assert!(result);

    let total_supply: U128 = view!(ft.ft_total_supply()).unwrap_json();

    assert_eq!(total_supply.0, 0);
}
