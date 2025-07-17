#!/bin/bash

# Usage: ./run_all_mutations.sh
# Run all mutations

for config in *_verified.conf; do
    echo "Running: $config"
    certoraMutate "$config"   
done