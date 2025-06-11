use soroban_sdk::{Address, BytesN, Env, Vec};
use access_control::access::AccessControl;
use access_control::management::{SingleAddressManagementTrait, MultipleAddressesManagementTrait};
use access_control::role::Role;
use access_control::transfer::TransferOwnershipTrait;
use access_control::emergency::get_emergency_mode;

// Structure to hold all system state
#[derive(Clone)]
pub struct SystemState {
    // Role addresses
    pub admin: Option<Address>,
    pub emergency_admin: Option<Address>,
    pub rewards_admin: Option<Address>,
    pub operations_admin: Option<Address>,
    pub pause_admin: Option<Address>,
    pub emergency_pause_admins: Vec<Address>,
    
    // Transfer deadlines
    pub admin_transfer_deadline: u64,
    pub em_admin_transfer_deadline: u64,
    
    // Future addresses
    pub future_admin: Option<Address>,
    pub future_em_admin: Option<Address>,
    
    // Emergency and upgrade
    pub emergency_mode: bool,
    pub upgrade_deadline: u64,
    pub future_wasm: Option<BytesN<32>>,
}

impl SystemState {
    pub fn read(env: &Env) -> Self {
        let access_control = AccessControl::new(env);
        
        // Read role addresses
        let admin = access_control.get_role_safe(&Role::Admin);
        let emergency_admin = access_control.get_role_safe(&Role::EmergencyAdmin);
        let rewards_admin = access_control.get_role_safe(&Role::RewardsAdmin);
        let operations_admin = access_control.get_role_safe(&Role::OperationsAdmin);
        let pause_admin = access_control.get_role_safe(&Role::PauseAdmin);
        let emergency_pause_admins = access_control.get_role_addresses(&Role::EmergencyPauseAdmin);
        
        // Read transfer deadlines
        let admin_transfer_deadline = access_control.get_transfer_ownership_deadline(&Role::Admin);
        let em_admin_transfer_deadline = access_control.get_transfer_ownership_deadline(&Role::EmergencyAdmin);
        
        // Read future addresses (only if deadline is set)
        let future_admin = if admin_transfer_deadline != 0 {
            // get_future_address will panic if deadline is 0, so we check first
            Some(access_control.get_future_address(&Role::Admin))
        } else {
            None
        };
        
        let future_em_admin = if em_admin_transfer_deadline != 0 {
            // get_future_address will panic if deadline is 0, so we check first
            Some(access_control.get_future_address(&Role::EmergencyAdmin))
        } else {
            None
        };
        
        // Read emergency mode
        let emergency_mode = get_emergency_mode(env);
        
        // Note: upgrade_deadline and future_wasm cannot be accessed directly
        // as upgrade::storage is private. These would need to be exposed
        // through public functions in the upgrade crate.
        let upgrade_deadline = 0;
        let future_wasm = None;
        
        Self {
            admin,
            emergency_admin,
            rewards_admin,
            operations_admin,
            pause_admin,
            emergency_pause_admins,
            admin_transfer_deadline,
            em_admin_transfer_deadline,
            future_admin,
            future_em_admin,
            emergency_mode,
            upgrade_deadline,
            future_wasm,
        }
    }
}