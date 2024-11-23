#!/bin/bash

# Create an output file
OUTPUT_FILE="output.log"
: > $OUTPUT_FILE

# Benchmark configurations
declare -a payloads=("big" "medium" "small")
declare -a services=("nginx_graphql" "tailcall_default" "tailcall_full_conf" "wundergraph_no_opt" "wundergraph_default" "apollo_router" "grafbase_default" "grafbase_cache")

# Run benchmarks and append results to the log
for payload in "${payloads[@]}"; do
  for service in "${services[@]}"; do
    echo "Running benchmark for payload: $payload, service: $service"
    ./benchmark.sh $payload $service | tee -a $OUTPUT_FILE
  done
done
