#!/bin/bash
# Eval script for Autoresearch loop

cd "$(dirname "$0")"

# Compile and run the project in release mode, capturing output
output=$(cargo run --release -q 2>&1)

# Check if the compilation/run failed
if [ $? -ne 0 ]; then
    echo "execution_time_ns: 999999999999" # Huge penalty for failing
    exit 0
fi

# Extract the execution time metric
echo "$output" | grep "^execution_time_ns:" || echo "execution_time_ns: 999999999999"