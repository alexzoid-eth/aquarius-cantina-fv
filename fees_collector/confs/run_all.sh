#!/bin/bash

# Usage: ./run_all.sh
# Run all mutations

for config in *_verified.conf; do
    echo "Running: $config"
    certoraSorobanProver "$config"   
done