use soroban_sdk::{contractimpl, Address, Env, Symbol, Vec};
use crate::contract::{FeesCollector, FeesCollectorArgs};
use crate::FeesCollectorClient;
use access_control::{
    access::{AccessControl, AccessControlTrait},
    emergency::{get_emergency_mode, set_emergency_mode},
    management::{SingleAddressManagementTrait, MultipleAddressesManagementTrait},
    role::{Role, SymbolRepresentation},
    transfer::TransferOwnershipTrait,
    utils::*,
};

// This extends FeesCollector with harness functions for testing access_control
#[contractimpl]
impl FeesCollector {

    pub fn h_init_all_roles(
        e: Env,
        admin: Address,
        emergency_admin: Address,
        rewards_admin: Address,
        operations_admin: Address,
        pause_admin: Address,
        emergency_pause_admins: Vec<Address>,
    ) {
        let access_control = AccessControl::new(&e);
        
        access_control.set_role_address(&Role::Admin, &admin);
        access_control.set_role_address(&Role::EmergencyAdmin, &emergency_admin);
        access_control.set_role_address(&Role::RewardsAdmin, &rewards_admin);
        access_control.set_role_address(&Role::OperationsAdmin, &operations_admin);
        access_control.set_role_address(&Role::PauseAdmin, &pause_admin);
        
        access_control.set_role_addresses(&Role::EmergencyPauseAdmin, &emergency_pause_admins);
    }

    pub fn h_get_role_safe(e: Env, role_name: Symbol) -> Option<Address> {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.get_role_safe(&role)
    }

    pub fn h_get_role(e: Env, role_name: Symbol) -> Address {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.get_role(&role)
    }

    pub fn h_set_role_address(e: Env, role_name: Symbol, address: Address) {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.set_role_address(&role, &address);
    }

    pub fn h_get_role_addresses(e: Env, role_name: Symbol) -> Vec<Address> {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.get_role_addresses(&role)
    }

    pub fn h_set_role_addresses(e: Env, role_name: Symbol, addresses: Vec<Address>) {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.set_role_addresses(&role, &addresses);
    }

    pub fn h_address_has_role(e: Env, address: Address, role_name: Symbol) -> bool {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.address_has_role(&address, &role)
    }

    pub fn h_assert_address_has_role(e: Env, address: Address, role_name: Symbol) {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.assert_address_has_role(&address, &role);
    }

    pub fn h_commit_transfer_ownership(e: Env, role_name: Symbol, future_address: Address) {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.commit_transfer_ownership(&role, &future_address);
    }

    pub fn h_apply_transfer_ownership(e: Env, role_name: Symbol) -> Address {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.apply_transfer_ownership(&role)
    }

    pub fn h_revert_transfer_ownership(e: Env, role_name: Symbol) {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.revert_transfer_ownership(&role);
    }

    pub fn h_get_transfer_ownership_dl(e: Env, role_name: Symbol) -> u64 {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.get_transfer_ownership_deadline(&role)
    }

    pub fn h_get_future_address(e: Env, role_name: Symbol) -> Address {
        let access_control = AccessControl::new(&e);
        let role = Role::from_symbol(&e, role_name);
        access_control.get_future_address(&role)
    }

    pub fn h_require_rewards_admin_or_owner(e: Env, address: Address) {
        require_rewards_admin_or_owner(&e, &address);
    }

    pub fn h_require_operations_admin_owner(e: Env, address: Address) {
        require_operations_admin_or_owner(&e, &address);
    }

    pub fn h_require_pause_emergency_admin(e: Env, address: Address) {
        require_pause_or_emergency_pause_admin_or_owner(&e, &address);
    }

    pub fn h_require_pause_admin_or_owner(e: Env, address: Address) {
        require_pause_admin_or_owner(&e, &address);
    }

    pub fn h_set_emergency_mode_direct(e: Env, value: bool) {
        set_emergency_mode(&e, &value);
    }

    pub fn h_get_emergency_mode_direct(e: Env) -> bool {
        get_emergency_mode(&e)
    }

    pub fn h_role_has_many_users(e: Env, role_name: Symbol) -> bool {
        let role = Role::from_symbol(&e, role_name);
        role.has_many_users()
    }

    pub fn h_role_is_transfer_delayed(e: Env, role_name: Symbol) -> bool {
        let role = Role::from_symbol(&e, role_name);
        role.is_transfer_delayed()
    }

    pub fn h_set_privileged_addrs(
        e: Env,
        rewards_admin: Address,
        operations_admin: Address,
        pause_admin: Address,
        emergency_pause_admins: Vec<Address>,
    ) {
        let access_control = AccessControl::new(&e);
        access_control.set_role_address(&Role::RewardsAdmin, &rewards_admin);
        access_control.set_role_address(&Role::OperationsAdmin, &operations_admin);
        access_control.set_role_address(&Role::PauseAdmin, &pause_admin);
        access_control.set_role_addresses(&Role::EmergencyPauseAdmin, &emergency_pause_admins);
    }
}