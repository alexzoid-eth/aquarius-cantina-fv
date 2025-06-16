#!/bin/bash

# Usage: ./run_all_permissions.sh
# Run all permissions verification configs

if [[ "$1" == "-h" ]] || [[ "$1" == "--help" ]]; then
    echo "Usage: $0"
    echo "Run all permissions verification configs for the fees_collector contract"
    exit 0
fi

# Run the four core permissions configs that provide complete coverage (split for timeout prevention)
configs=(
    "permissions_admin_role_management_verified.conf"
    "permissions_transfer_deadlines_verified.conf"
    "permissions_future_addresses_verified.conf"
    "permissions_emergency_and_upgrades_verified.conf"
)

for config in "${configs[@]}"; do
    echo "Running: $config"
    certoraSorobanProver "$config"    
done