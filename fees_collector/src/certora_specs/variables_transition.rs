// Variable transition properties

use soroban_sdk::Env;
use cvlr::cvlr_assert;
use crate::certora_specs::base::ParametricParams;
use ghost_state::GhostState;

// Check deadline lifecycle: 0 -> timestamp -> 0
fn check_deadline_lifecycle(
    e: &Env,
    deadline_before: u64,
    deadline_after: u64,
    _future_before: Option<impl Clone>,
    future_after: Option<impl Clone>,
) -> bool {
    if deadline_before == 0 && deadline_after != 0 {
        // Transitwion from 0 to non-zero (commit)
        deadline_after > e.ledger().timestamp() && future_after.is_some()
    } else if deadline_before != 0 && deadline_after == 0 {
        // Transition from non-zero to 0 (apply or revert)
        future_after.is_none()
    } else if deadline_before != 0 && deadline_after != 0 {
        // Cannot change non-zero deadline to different non-zero value
        deadline_before == deadline_after
    } else {
        true
    }
}

// Check admin deadline lifecycle
pub fn variables_transition_admin_deadline_lifecycle(
    e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = check_deadline_lifecycle(
        e,
        before.admin_transfer_deadline,
        after.admin_transfer_deadline,
        before.future_admin,
        after.future_admin,
    );
    
    cvlr_assert!(valid);
}

// Check emergency admin deadline lifecycle
pub fn variables_transition_em_admin_deadline_lifecycle(
    e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = check_deadline_lifecycle(
        e,
        before.em_admin_transfer_deadline,
        after.em_admin_transfer_deadline,
        before.future_em_admin,
        after.future_em_admin,
    );
    
    cvlr_assert!(valid);
}

// Check upgrade deadline lifecycle
pub fn variables_transition_upgrade_deadline_lifecycle(
    e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = check_deadline_lifecycle(
        e,
        before.upgrade_deadline,
        after.upgrade_deadline,
        before.future_wasm,
        after.future_wasm,
    );
    
    cvlr_assert!(valid);
}

// Check mutual exclusion of ownership transfers and upgrades
pub fn variables_transition_mutual_exclusion(
    _e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    // Count active deadlines before and after
    let before_active = (if before.admin_transfer_deadline != 0 { 1 } else { 0 }) +
                       (if before.em_admin_transfer_deadline != 0 { 1 } else { 0 }) +
                       (if before.upgrade_deadline != 0 { 1 } else { 0 });
    
    let after_active = (if after.admin_transfer_deadline != 0 { 1 } else { 0 }) +
                      (if after.em_admin_transfer_deadline != 0 { 1 } else { 0 }) +
                      (if after.upgrade_deadline != 0 { 1 } else { 0 });
    
    // Mutual exclusion: at most one can be active at a time
    let valid = after_active <= 1 && 
                // If multiple are active after, they must have been active before (no new commits)
                (after_active <= before_active || after_active <= 1);
    
    cvlr_assert!(valid);
}

// Check time-gated role changes for Admin
pub fn variables_transition_admin_role(
    e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = if before.admin != after.admin {
        if before.admin.is_none() {
            // Initial setting is allowed
            after.admin.is_some()
        } else {
            // Must have gone through apply process
            before.admin_transfer_deadline != 0 &&
            after.admin_transfer_deadline == 0 &&
            e.ledger().timestamp() >= before.admin_transfer_deadline
        }
    } else {
        true
    };
    
    cvlr_assert!(valid);
}

// Check time-gated role changes for Emergency Admin
pub fn variables_transition_emergency_admin_role(
    e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = if before.emergency_admin != after.emergency_admin {
        if before.emergency_admin.is_some() {
            before.em_admin_transfer_deadline != 0 &&
            after.em_admin_transfer_deadline == 0 &&
            e.ledger().timestamp() >= before.em_admin_transfer_deadline
        } else {
            // Initial setting is allowed
            true
        }
    } else {
        true
    };
    
    cvlr_assert!(valid);
}

// Check future admin value consistency
pub fn variables_transition_future_admin_consistency(
    _e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = if before.admin_transfer_deadline == after.admin_transfer_deadline && 
                   after.admin_transfer_deadline != 0 {
        // If deadline didn't change and is non-zero, future value must remain same
        before.future_admin == after.future_admin
    } else {
        true
    };
    
    cvlr_assert!(valid);
}

// Check future emergency admin value consistency
pub fn variables_transition_future_em_admin_consistency(
    _e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = if before.em_admin_transfer_deadline == after.em_admin_transfer_deadline && 
                   after.em_admin_transfer_deadline != 0 {
        // If deadline didn't change and is non-zero, future value must remain same
        before.future_em_admin == after.future_em_admin
    } else {
        true
    };
    
    cvlr_assert!(valid);
}

// Check future wasm value consistency
pub fn variables_transition_future_wasm_consistency(
    _e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = if before.upgrade_deadline == after.upgrade_deadline && 
                   after.upgrade_deadline != 0 {
        // If deadline didn't change and is non-zero, future value must remain same
        before.future_wasm == after.future_wasm
    } else {
        true
    };
    
    cvlr_assert!(valid);
}

// Check emergency mode effects on upgrades vs ownership transfers
pub fn variables_transition_emergency_mode_behavior(
    e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    let valid = if before.emergency_mode && 
                   before.upgrade_deadline != 0 && 
                   after.upgrade_deadline == 0 {
        // In emergency mode, upgrade can be applied immediately
        // But admin transfer still requires time delay
        if before.admin_transfer_deadline != 0 && after.admin_transfer_deadline == 0 {
            e.ledger().timestamp() >= before.admin_transfer_deadline
        } else {
            true
        }
    } else {
        true
    };
    
    cvlr_assert!(valid);
}

// Check that initial admin can only be set once
pub fn variables_transition_admin_init_once(
    _e: &Env,
    _params: &ParametricParams,
    call_fn: impl FnOnce()
) {
    let before = GhostState::read();
    call_fn();
    let after = GhostState::read();
    
    // If admin was already set, it can only change through transfer process
    let valid = if before.admin.is_some() && before.admin != after.admin {
        // Must have gone through the transfer process
        before.admin_transfer_deadline != 0
    } else {
        true
    };
    
    cvlr_assert!(valid);
}
