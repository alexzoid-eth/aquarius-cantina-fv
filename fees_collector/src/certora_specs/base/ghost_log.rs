// Log GhostState fields

use soroban_sdk::{Address, BytesN};
use cvlr::clog;
use ghost_state::GhostState;

pub fn log_deadline(value: u64, _name: &str) {
    clog!(value);
}

pub fn log_address_option(addr: &Option<Address>) {
    match addr {
        Some(_address) => {
            let val = 1;
            clog!(val);
        }
        None => {
            let val = 0;
            clog!(val);
        }
    }
}

pub fn log_wasm_option(wasm: &Option<BytesN<32>>) {
    match wasm {
        Some(_) => {
            let val = 1;
            clog!(val);
        }
        None => {
            let val = 0;
            clog!(val);
        }
    }
}

pub fn log_emergency_pause_admins_count(count: u32) {
    clog!(count);
}

pub fn ghost_log_details() {
    let state = GhostState::read();
    
    log_address_option(&state.admin);
    log_address_option(&state.emergency_admin);
    log_address_option(&state.rewards_admin);
    log_address_option(&state.operations_admin);
    log_address_option(&state.pause_admin);
    
    log_emergency_pause_admins_count(state.emergency_pause_admins.len());
    
    log_deadline(state.admin_transfer_deadline, "admin_transfer_deadline");
    log_deadline(state.em_admin_transfer_deadline, "em_admin_transfer_deadline");
    
    log_address_option(&state.future_admin);
    log_address_option(&state.future_em_admin);
    
    clog!(state.emergency_mode);
    
    log_deadline(state.upgrade_deadline, "upgrade_deadline");
    log_wasm_option(&state.future_wasm);
}