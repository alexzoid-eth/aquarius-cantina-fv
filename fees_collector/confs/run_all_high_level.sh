#!/bin/bash

# Usage: ./run_all_high_level.sh
# Run all high-level verification configs

if [[ "$1" == "-h" ]] || [[ "$1" == "--help" ]]; then
    echo "Usage: $0"
    echo "Run all high-level scenario verification configs for the fees_collector contract"
    exit 0
fi

for config in high_level_*.conf; do
    echo "Running: $config"
    certoraSorobanProver "$config"    
done