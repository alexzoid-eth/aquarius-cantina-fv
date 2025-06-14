#!/bin/bash

configs=(
    "fees_collector_variables_transition_admin_transfer_deadlines_verified.conf"
    "fees_collector_variables_transition_upgrade_deadlines_verified.conf"
    "fees_collector_variables_transition_role_transitions_verified.conf"
    "fees_collector_variables_transition_future_consistency_verified.conf"
    "fees_collector_variables_transition_emergency_mode_verified.conf"
    "fees_collector_variables_transition_admin_init_once_verified.conf"
)

for config in "${configs[@]}"; do
    certoraSorobanProver "$config"    
done