#!/bin/bash

# Usage: ./run_all_state_transition.sh
# Run all state transition verification configs

if [[ "$1" == "-h" ]] || [[ "$1" == "--help" ]]; then
    echo "Usage: $0"
    echo "Run all state transition verification configs for the fees_collector contract"
    exit 0
fi

configs=(
    "state_transition_admin_transfer_deadlines_verified.conf"
    "state_transition_upgrade_deadlines_verified.conf"
    "state_transition_role_transitions_verified.conf"
    "state_transition_future_consistency_verified.conf"
    "state_transition_emergency_mode_verified.conf"
    "state_transition_admin_init_once_verified.conf"
)

for config in "${configs[@]}"; do
    echo "Running: $config"
    certoraSorobanProver "$config"    
done