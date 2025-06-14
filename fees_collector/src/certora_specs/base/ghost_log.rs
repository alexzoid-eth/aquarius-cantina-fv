// Log GhostState fields

use cvlr::clog;
use ghost_state::GhostState;

pub fn ghost_log_all() {
    let state = GhostState::read();
    
    // Role addresses - log as 1 if Some, 0 if None
    let admin_exists = match &state.admin {
        Some(_) => 1u32,
        None => 0u32,
    };
    clog!(admin_exists);
    
    let emergency_admin_exists = match &state.emergency_admin {
        Some(_) => 1u32,
        None => 0u32,
    };
    clog!(emergency_admin_exists);
    
    let rewards_admin_exists = match &state.rewards_admin {
        Some(_) => 1u32,
        None => 0u32,
    };
    clog!(rewards_admin_exists);
    
    let operations_admin_exists = match &state.operations_admin {
        Some(_) => 1u32,
        None => 0u32,
    };
    clog!(operations_admin_exists);
    
    let pause_admin_exists = match &state.pause_admin {
        Some(_) => 1u32,
        None => 0u32,
    };
    clog!(pause_admin_exists);
    
    let emergency_pause_admins_count = state.emergency_pause_admins.len();
    clog!(emergency_pause_admins_count);
    
    // Transfer deadlines
    let admin_transfer_deadline = state.admin_transfer_deadline;
    clog!(admin_transfer_deadline);
    
    let em_admin_transfer_deadline = state.em_admin_transfer_deadline;
    clog!(em_admin_transfer_deadline);
    
    // Future addresses
    let future_admin_exists = match &state.future_admin {
        Some(_) => 1u32,
        None => 0u32,
    };
    clog!(future_admin_exists);
    
    let future_em_admin_exists = match &state.future_em_admin {
        Some(_) => 1u32,
        None => 0u32,
    };
    clog!(future_em_admin_exists);
    
    // Emergency mode
    let emergency_mode = state.emergency_mode;
    clog!(emergency_mode);
    
    // Upgrade state
    let upgrade_deadline = state.upgrade_deadline;
    clog!(upgrade_deadline);
    
    let future_wasm_exists = match &state.future_wasm {
        Some(_) => 1u32,
        None => 0u32,
    };
    clog!(future_wasm_exists);
}