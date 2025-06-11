// Generate parametric rules

#[macro_export]
macro_rules! parametric_rule {
    ($f:ident) => {
        paste::paste! {
            // AdminInterface
            #[rule]
            pub fn [< $f _init_admin >](
                e: Env,
                account: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(account.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::init_admin(e.clone(), account.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // UpgradeableContract - version
            #[rule]
            pub fn [< $f _version >](e: Env) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e);
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::version();
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // UpgradeableContract - commit_upgrade
            #[rule]
            pub fn [< $f _commit_upgrade >](
                e: Env,
                admin: Address,
                new_wasm_hash: BytesN<32>
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(admin.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::commit_upgrade(e.clone(), admin.clone(), new_wasm_hash.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // UpgradeableContract - apply_upgrade
            #[rule]
            pub fn [< $f _apply_upgrade >](
                e: Env,
                admin: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(admin.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::apply_upgrade(e.clone(), admin.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // UpgradeableContract - revert_upgrade
            #[rule]
            pub fn [< $f _revert_upgrade >](
                e: Env,
                admin: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(admin.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::revert_upgrade(e.clone(), admin.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // UpgradeableContract - set_emergency_mode
            #[rule]
            pub fn [< $f _set_emergency_mode >](
                e: Env,
                emergency_admin: Address,
                value: bool
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(emergency_admin.clone())
                    .with_value(value);
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::set_emergency_mode(e.clone(), emergency_admin.clone(), value);
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // UpgradeableContract - get_emergency_mode
            #[rule]
            pub fn [< $f _get_emergency_mode >](e: Env) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e);
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::get_emergency_mode(e.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // TransferableContract - commit_transfer_ownership
            #[rule]
            pub fn [< $f _commit_transfer_ownership >](
                e: Env,
                admin: Address,
                role_name: Symbol,
                new_address: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(admin.clone())
                    .with_role(role_name.clone())
                    .with_new_address(new_address.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::commit_transfer_ownership(e.clone(), admin.clone(), role_name.clone(), new_address.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // TransferableContract - apply_transfer_ownership
            #[rule]
            pub fn [< $f _apply_transfer_ownership >](
                e: Env,
                admin: Address,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(admin.clone())
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::apply_transfer_ownership(e.clone(), admin.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // TransferableContract - revert_transfer_ownership
            #[rule]
            pub fn [< $f _revert_transfer_ownership >](
                e: Env,
                admin: Address,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(admin.clone())
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::revert_transfer_ownership(e.clone(), admin.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // TransferableContract - get_future_address
            #[rule]
            pub fn [< $f _get_future_address >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::get_future_address(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }
            
            //
            // access_control_harness
            //
            
            // h_init_all_roles
            #[rule]
            pub fn [< $f _h_init_all_roles >](
                e: Env,
                admin: Address,
                emergency_admin: Address,
                rewards_admin: Address,
                operations_admin: Address,
                pause_admin: Address,
                emergency_pause_admins: Vec<Address>
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(admin.clone())
                    .with_emergency_admin(emergency_admin.clone())
                    .with_rewards_admin(rewards_admin.clone())
                    .with_operations_admin(operations_admin.clone())
                    .with_pause_admin(pause_admin.clone())
                    .with_emergency_pause_admins(emergency_pause_admins.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_init_all_roles(
                        e.clone(), 
                        admin.clone(), 
                        emergency_admin.clone(),
                        rewards_admin.clone(),
                        operations_admin.clone(),
                        pause_admin.clone(),
                        emergency_pause_admins.clone()
                    );
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_get_role_safe
            #[rule]
            pub fn [< $f _h_get_role_safe >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_get_role_safe(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_get_role
            #[rule]
            pub fn [< $f _h_get_role >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_get_role(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_set_role_address
            #[rule]
            pub fn [< $f _h_set_role_address >](
                e: Env,
                role_name: Symbol,
                address: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone())
                    .with_new_address(address.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_set_role_address(e.clone(), role_name.clone(), address.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_get_role_addresses
            #[rule]
            pub fn [< $f _h_get_role_addresses >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_get_role_addresses(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_set_role_addresses
            #[rule]
            pub fn [< $f _h_set_role_addresses >](
                e: Env,
                role_name: Symbol,
                addresses: Vec<Address>
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone())
                    .with_addresses(addresses.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_set_role_addresses(e.clone(), role_name.clone(), addresses.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_address_has_role
            #[rule]
            pub fn [< $f _h_address_has_role >](
                e: Env,
                address: Address,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(address.clone())
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_address_has_role(e.clone(), address.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_assert_address_has_role
            #[rule]
            pub fn [< $f _h_assert_address_has_role >](
                e: Env,
                address: Address,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(address.clone())
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_assert_address_has_role(e.clone(), address.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_commit_transfer_ownership
            #[rule]
            pub fn [< $f _h_commit_transfer_ownership >](
                e: Env,
                role_name: Symbol,
                future_address: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone())
                    .with_new_address(future_address.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_commit_transfer_ownership(e.clone(), role_name.clone(), future_address.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_apply_transfer_ownership
            #[rule]
            pub fn [< $f _h_apply_transfer_ownership >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_apply_transfer_ownership(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_revert_transfer_ownership
            #[rule]
            pub fn [< $f _h_revert_transfer_ownership >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_revert_transfer_ownership(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_get_transfer_ownership_dl
            #[rule]
            pub fn [< $f _h_get_transfer_ownership_dl >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_get_transfer_ownership_dl(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_get_future_address
            #[rule]
            pub fn [< $f _h_get_future_address >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_get_future_address(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_require_rewards_admin_or_owner
            #[rule]
            pub fn [< $f _h_require_rewards_admin_or_owner >](
                e: Env,
                address: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(address.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_require_rewards_admin_or_owner(e.clone(), address.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_require_operations_admin_owner
            #[rule]
            pub fn [< $f _h_require_operations_admin_owner >](
                e: Env,
                address: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(address.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_require_operations_admin_owner(e.clone(), address.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_require_pause_emergency_admin
            #[rule]
            pub fn [< $f _h_require_pause_emergency_admin >](
                e: Env,
                address: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(address.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_require_pause_emergency_admin(e.clone(), address.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_require_pause_admin_or_owner
            #[rule]
            pub fn [< $f _h_require_pause_admin_or_owner >](
                e: Env,
                address: Address
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_caller(address.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_require_pause_admin_or_owner(e.clone(), address.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_set_emergency_mode_direct
            #[rule]
            pub fn [< $f _h_set_emergency_mode_direct >](
                e: Env,
                value: bool
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_value(value);
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_set_emergency_mode_direct(e.clone(), value);
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_get_emergency_mode_direct
            #[rule]
            pub fn [< $f _h_get_emergency_mode_direct >](e: Env) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e);
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_get_emergency_mode_direct(e.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_role_has_many_users
            #[rule]
            pub fn [< $f _h_role_has_many_users >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_role_has_many_users(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_role_is_transfer_delayed
            #[rule]
            pub fn [< $f _h_role_is_transfer_delayed >](
                e: Env,
                role_name: Symbol
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_role(role_name.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    let _result = FeesCollector::h_role_is_transfer_delayed(e.clone(), role_name.clone());
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }

            // h_set_privileged_addrs
            #[rule]
            pub fn [< $f _h_set_privileged_addrs >](
                e: Env,
                rewards_admin: Address,
                operations_admin: Address,
                pause_admin: Address,
                emergency_pause_admins: Vec<Address>
            ) {
                init_verification!(e);
                
                let params = ParametricParams::new(&e)
                    .with_rewards_admin(rewards_admin.clone())
                    .with_operations_admin(operations_admin.clone())
                    .with_pause_admin(pause_admin.clone())
                    .with_emergency_pause_admins(emergency_pause_admins.clone());
                
                let call_fn = || { 
                    log_state_details(e.clone());
                    FeesCollector::h_set_privileged_addrs(
                        e.clone(),
                        rewards_admin.clone(),
                        operations_admin.clone(),
                        pause_admin.clone(),
                        emergency_pause_admins.clone()
                    );
                    log_state_details(e.clone());
                };
                
                $f(&e, &params, call_fn);
            }
        }
    };
}