use soroban_sdk::{Env, Address, BytesN, Symbol, Vec};
use cvlr::asserts::cvlr_assert;
use cvlr_soroban_derive::rule;
use access_control::constants::ADMIN_ACTIONS_DELAY;

use crate::contract::FeesCollector;
use crate::interface::AdminInterface;
use access_control::interface::TransferableContract;
use upgrade::interface::UpgradeableContract;

#[rule]
pub fn setter_getter_emergency_mode(e: Env, emergency_admin: Address, value: u8) {
    
    // @note It seems there is an internal prover error when using bool in parameters natively
    //  https://prover.certora.com/output/52567/2eefc1e0560243cdb210dcb145f7d99d/?anonymousKey=418550b6f8be9074073cc1bb4d47ae317905a6cd
    // Convert u8 to bool: non-zero == true, zero == false
    let bool_value = value != 0;

    FeesCollector::set_emergency_mode(e.clone(), emergency_admin, bool_value);
    let result = FeesCollector::get_emergency_mode(e);
    cvlr_assert!(result == bool_value);
}

#[rule]
pub fn setter_getter_init_admin(e: Env, admin: Address) {
    FeesCollector::init_admin(e.clone(), admin.clone());
    let result = FeesCollector::h_get_role(e.clone(), Symbol::new(&e, "Admin"));
    cvlr_assert!(result == admin);
}

#[rule]
pub fn setter_getter_future_address(e: Env, admin: Address, role_name: Symbol, new_address: Address) {
    FeesCollector::commit_transfer_ownership(e.clone(), admin, role_name.clone(), new_address.clone());
    let result = FeesCollector::get_future_address(e, role_name);
    cvlr_assert!(result == new_address);
}

#[rule]
pub fn setter_getter_commit_transfer_deadline(e: Env, admin: Address, role_name: Symbol, new_address: Address) {
    let deadline = e.ledger().timestamp() + ADMIN_ACTIONS_DELAY;
    FeesCollector::commit_transfer_ownership(e.clone(), admin, role_name.clone(), new_address.clone());
    let result = FeesCollector::h_get_transfer_ownership_dl(e, role_name);
    cvlr_assert!(result == deadline);
}

#[rule]
pub fn setter_getter_role_address_single(e: Env, role_name: Symbol, address: Address) {
    FeesCollector::h_set_role_address(e.clone(), role_name.clone(), address.clone());
    let result = FeesCollector::h_get_role(e, role_name);
    cvlr_assert!(result == address);
}

#[rule]
pub fn setter_getter_role_addresses_multiple(e: Env, role_name: Symbol, addresses: Vec<Address>) {
    FeesCollector::h_set_role_addresses(e.clone(), role_name.clone(), addresses.clone());
    let result = FeesCollector::h_get_role_addresses(e, role_name);
    cvlr_assert!(result == addresses);
}

#[rule]
pub fn setter_getter_commit_upgrade_future_wasm(e: Env, admin: Address, new_wasm_hash: BytesN<32>) {
    FeesCollector::commit_upgrade(e.clone(), admin, new_wasm_hash.clone());
    let future_wasm = FeesCollector::h_get_future_wasm(e);
    cvlr_assert!(future_wasm == Some(new_wasm_hash));
}

#[rule]
pub fn setter_getter_commit_upgrade_deadline(e: Env, admin: Address, new_wasm_hash: BytesN<32>) {
    FeesCollector::commit_upgrade(e.clone(), admin, new_wasm_hash);
    let deadline = FeesCollector::h_get_upgrade_deadline(e);
    cvlr_assert!(deadline > 0);
}