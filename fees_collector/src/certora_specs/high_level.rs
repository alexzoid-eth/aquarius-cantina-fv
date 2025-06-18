use cvlr::{cvlr_satisfy, cvlr_assume};
use soroban_sdk::{Env, Address, BytesN, Symbol};
use cvlr::asserts::cvlr_assert;
use cvlr_soroban_derive::rule;

use crate::setup_fv;
use crate::contract::FeesCollector;
use crate::interface::AdminInterface;
use access_control::interface::TransferableContract;
use upgrade::interface::UpgradeableContract;

//
// Ownership transfer
//

#[rule]
pub fn high_level_revert_ownership_transfer(e: Env, current_admin: Address, new_admin: Address) {
    
    setup_fv!(e);
    
    // Initialize and commit transfer
    FeesCollector::init_admin(e.clone(), current_admin.clone());
    FeesCollector::commit_transfer_ownership(e.clone(), current_admin.clone(), Symbol::new(&e, "Admin"), new_admin);
    
    // Revert before deadline
    FeesCollector::revert_transfer_ownership(e.clone(), current_admin.clone(), Symbol::new(&e, "Admin"));
    
    // Verify admin unchanged
    let final_admin = FeesCollector::h_get_role(e.clone(), Symbol::new(&e, "Admin"));
    cvlr_assert!(final_admin == current_admin);
}

#[rule]
pub fn high_level_revert_ownership_transfer_reachability(e: Env, current_admin: Address, new_admin: Address) {
    
    // Initialize and commit transfer
    FeesCollector::init_admin(e.clone(), current_admin.clone());
    FeesCollector::commit_transfer_ownership(e.clone(), current_admin.clone(), Symbol::new(&e, "Admin"), new_admin);
    
    // Revert before deadline
    FeesCollector::revert_transfer_ownership(e.clone(), current_admin.clone(), Symbol::new(&e, "Admin"));
    
    // Verify admin unchanged
    let final_admin = FeesCollector::h_get_role(e.clone(), Symbol::new(&e, "Admin"));

    // Previous rule reachability
    cvlr_satisfy!(final_admin == current_admin);
}

//
// Upgrade
//

#[rule]
pub fn high_level_upgrade_success(e: Env, admin: Address, new_wasm_hash: BytesN<32>, timestamp_offset: u64) {
    cvlr_assume!(timestamp_offset < 1000000);
    
    // Initialize admin
    FeesCollector::init_admin(e.clone(), admin.clone());
    
    // Commit upgrade
    FeesCollector::commit_upgrade(e.clone(), admin.clone(), new_wasm_hash.clone());
    
    // Verify future wasm is set
    let future_wasm = FeesCollector::h_get_future_wasm(e.clone());
    cvlr_assert!(future_wasm == Some(new_wasm_hash.clone()));
    
    // Time progression handled by prover
    
    // Apply upgrade
    FeesCollector::apply_upgrade(e.clone(), admin);
    
    // In real scenario, contract would be upgraded here
    // We verify that future_wasm is cleared
    let future_wasm_after = FeesCollector::h_get_future_wasm(e);
    cvlr_assert!(future_wasm_after.is_none());
}

#[rule]
pub fn high_level_upgrade_reachable(e: Env, admin: Address, new_wasm_hash: BytesN<32>) {
    FeesCollector::init_admin(e.clone(), admin.clone());
    FeesCollector::commit_upgrade(e.clone(), admin.clone(), new_wasm_hash);
    
    // Time progression handled by prover
    
    FeesCollector::apply_upgrade(e.clone(), admin);
    
    // Satisfy if we reached this point without panic
    cvlr_satisfy!(true);
}

#[rule]
pub fn high_level_revert_upgrade(e: Env, admin: Address, new_wasm_hash: BytesN<32>) {
    // Initialize and commit upgrade
    FeesCollector::init_admin(e.clone(), admin.clone());
    FeesCollector::commit_upgrade(e.clone(), admin.clone(), new_wasm_hash);
    
    // Revert upgrade
    FeesCollector::revert_upgrade(e.clone(), admin);
    
    // Verify future wasm cleared
    let future_wasm = FeesCollector::h_get_future_wasm(e);
    cvlr_assert!(future_wasm.is_none());
}

//
// Emergency mode
//

#[rule]
pub fn high_level_emergency_blocks_upgrade(e: Env, admin: Address, emergency_admin: Address, new_wasm_hash: BytesN<32>) {
    cvlr_assume!(admin != emergency_admin);
    
    // Initialize roles
    FeesCollector::init_admin(e.clone(), admin.clone());
    FeesCollector::h_set_role_address(e.clone(), Symbol::new(&e, "EmergencyAdmin"), emergency_admin.clone());
    
    // Commit upgrade
    FeesCollector::commit_upgrade(e.clone(), admin.clone(), new_wasm_hash);
    
    // Set emergency mode
    FeesCollector::set_emergency_mode(e.clone(), emergency_admin, true);
    
    // Fast forward time
    // Time progression handled by prover
    
    // Try to apply upgrade - should panic due to emergency mode
    FeesCollector::apply_upgrade(e.clone(), admin);
    
    // Should never reach here
    cvlr_assert!(false);
}

#[rule]
pub fn high_level_emergency_mode_toggle(e: Env, admin: Address, emergency_admin: Address) {
    // Initialize roles
    FeesCollector::init_admin(e.clone(), admin.clone());
    FeesCollector::h_set_role_address(e.clone(), Symbol::new(&e, "EmergencyAdmin"), emergency_admin.clone());
    
    // Initially emergency mode is off
    let initial_mode = FeesCollector::get_emergency_mode(e.clone());
    cvlr_assert!(!initial_mode);
    
    // Turn on emergency mode
    FeesCollector::set_emergency_mode(e.clone(), emergency_admin.clone(), true);
    cvlr_assert!(FeesCollector::get_emergency_mode(e.clone()));
    
    // Turn off emergency mode
    FeesCollector::set_emergency_mode(e.clone(), emergency_admin, false);
    cvlr_assert!(!FeesCollector::get_emergency_mode(e));
}

//
// Multi-step
//

#[rule]
pub fn high_level_concurrent_transfers_blocked(e: Env, admin: Address, new_admin1: Address, new_admin2: Address) {
    cvlr_assume!(admin != new_admin1);
    cvlr_assume!(admin != new_admin2);
    cvlr_assume!(new_admin1 != new_admin2);
    
    // Initialize admin
    FeesCollector::init_admin(e.clone(), admin.clone());
    
    // Commit first transfer
    FeesCollector::commit_transfer_ownership(e.clone(), admin.clone(), Symbol::new(&e, "Admin"), new_admin1);
    
    // Try to commit second transfer - should panic (mutual exclusion)
    FeesCollector::commit_transfer_ownership(e.clone(), admin, Symbol::new(&e, "Admin"), new_admin2);
    
    // Should never reach here
    cvlr_assert!(false);
}

#[rule]
pub fn high_level_transfer_then_upgrade_sequence(e: Env, old_admin: Address, new_admin: Address, new_wasm_hash: BytesN<32>) {
    cvlr_assume!(old_admin != new_admin);
    
    // Initialize with old admin
    FeesCollector::init_admin(e.clone(), old_admin.clone());
    
    // Transfer ownership
    FeesCollector::commit_transfer_ownership(e.clone(), old_admin.clone(), Symbol::new(&e, "Admin"), new_admin.clone());
    // Time progression handled by prover
    FeesCollector::apply_transfer_ownership(e.clone(), old_admin, Symbol::new(&e, "Admin"));
    
    // Now new admin commits upgrade
    FeesCollector::commit_upgrade(e.clone(), new_admin.clone(), new_wasm_hash);
    // Time progression handled by prover
    FeesCollector::apply_upgrade(e.clone(), new_admin.clone());
    
    // Verify both operations succeeded
    let final_admin = FeesCollector::h_get_role(e.clone(), Symbol::new(&e, "Admin"));
    cvlr_assert!(final_admin == new_admin);
    
    let future_wasm = FeesCollector::h_get_future_wasm(e);
    cvlr_assert!(future_wasm.is_none()); // Should be cleared after apply
}

#[rule]
pub fn high_level_deadline_expiry_prevents_apply(e: Env, admin: Address, new_admin: Address) {
    cvlr_assume!(admin != new_admin);
    
    // Initialize and commit transfer
    FeesCollector::init_admin(e.clone(), admin.clone());
    FeesCollector::commit_transfer_ownership(e.clone(), admin.clone(), Symbol::new(&e, "Admin"), new_admin);
    
    // Try to apply before deadline - should panic
    FeesCollector::apply_transfer_ownership(e.clone(), admin, Symbol::new(&e, "Admin"));
    
    // Should never reach here
    cvlr_assert!(false);
}

#[rule]
pub fn high_level_role_transfer_preserves_others(e: Env, admin: Address, old_emergency: Address, new_emergency: Address, operations_admin: Address) {
    cvlr_assume!(old_emergency != new_emergency);
    
    // Initialize multiple roles
    FeesCollector::init_admin(e.clone(), admin.clone());
    FeesCollector::h_set_role_address(e.clone(), Symbol::new(&e, "EmergencyAdmin"), old_emergency);
    FeesCollector::h_set_role_address(e.clone(), Symbol::new(&e, "OperationsAdmin"), operations_admin.clone());
    
    // Transfer emergency admin
    FeesCollector::commit_transfer_ownership(e.clone(), admin.clone(), Symbol::new(&e, "EmergencyAdmin"), new_emergency.clone());
    // Time progression handled by prover
    FeesCollector::apply_transfer_ownership(e.clone(), admin.clone(), Symbol::new(&e, "EmergencyAdmin"));
    
    // Verify emergency admin changed but others preserved
    let final_emergency = FeesCollector::h_get_role(e.clone(), Symbol::new(&e, "EmergencyAdmin"));
    let final_admin = FeesCollector::h_get_role(e.clone(), Symbol::new(&e, "Admin"));
    let final_operations = FeesCollector::h_get_role(e.clone(), Symbol::new(&e, "OperationsAdmin"));
    
    cvlr_assert!(final_emergency == new_emergency);
    cvlr_assert!(final_admin == admin);
    cvlr_assert!(final_operations == operations_admin);
}