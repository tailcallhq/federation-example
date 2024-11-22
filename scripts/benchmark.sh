#!/bin/bash

benchmark_file="bench-hey-$1.json"
echo "Script executed from: ${PWD}"

# Define a function to kill processes running on a specific port
kill_port_process() {
    local port=$1
    echo "Checking for process on port $port..."
    pid=$(lsof -ti:$port)
    if [ -n "$pid" ]; then
        echo "Killing process $pid running on port $port..."
        kill -9 "$pid"
        echo "Process terminated."
    else
        echo "No process found on port $port."
    fi
}
cleanup() {
    echo "Cleaning up..."
    # List of processes to terminate
    processes=("tailcall" "nginx" "wunder" "apollo" "grafbase")

    for process in "${processes[@]}"; do
        for pid in $(ls /proc | grep -E '^[0-9]+$' | xargs -I {} sh -c "grep -l '$process' /proc/{}/comm 2>/dev/null" | sed 's|/proc/||g' | sed 's|/comm||g'); do
            echo "Terminating $process with PID $pid..."
            kill -9 "$pid" 2>/dev/null || echo "Failed to terminate PID $pid"
        done
    done
    kill_port_process 4006
    kill_port_process 8090
    kill_port_process 8030
    echo "All processes terminated, ports freed."
}

# Trap to ensure cleanup on script exit
trap cleanup EXIT

# Start the server or application based on project
cargo run --release $1 &
sleep 1

echo "Setup Benchmark (payload=$1) (project=$2)"

source_port="4006"
nginx_port="8090"

echo "Running Benchmark (payload=$1) (project=$2)"

if [ "$2" = "source_graphql" ]; then
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:$source_port/graphql
elif [ "$2" = "source_rest_api" ]; then
    hey -n 200 -z 10s -m GET -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:$source_port/$1-json
elif [ "$2" = "nginx_graphql" ]; then
    nginx
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:$nginx_port/graphql
elif [ "$2" = "nginx_rest_api" ]; then
    nginx
    sleep 5
    hey -n 200 -z 10s -m GET -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:$nginx_port/$1-json
elif [ "$2" = "tailcall_default" ]; then
    TC_LOG_LEVEL=error tailcall start ./1-basic.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:8030/graphql
elif [ "$2" = "tailcall_tweaks" ]; then
    TC_LOG_LEVEL=error tailcall start ./2-http-tweaks.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:8030/graphql
elif [ "$2" = "tailcall_http_cache" ]; then
    TC_LOG_LEVEL=error tailcall start ./3-http-cache.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:8030/graphql
elif [ "$2" = "tailcall_cache_dir" ]; then
    TC_LOG_LEVEL=error tailcall start ./4-http-cache-directive.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:8030/graphql
elif [ "$2" = "tailcall_dedupe_op" ]; then
    TC_LOG_LEVEL=error tailcall start ./5-dedupe.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:8030/graphql
elif [ "$2" = "tailcall_full_conf" ]; then
    TC_LOG_LEVEL=error tailcall start ./6-all.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:8030/graphql
elif [ "$2" = "wundergraph_no_opt" ]; then
    ./wunder --config wundergraph_1_basic.json > /dev/null &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:3002/graphql
elif [ "$2" = "wundergraph_dedupe" ]; then
    ./wunder --config wundergraph_2_dedupe.json > /dev/null &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:3002/graphql
elif [ "$2" = "wundergraph_default" ]; then
    ./wunder --config wundergraph_3_cached.json > /dev/null &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:3002/graphql
elif [ "$2" = "apollo_router" ]; then
    ./apollo -s supergraph.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:4000
elif [ "$2" = "grafbase_default" ]; then
    ./grafbase -c grafbase_1_basic.toml -s supergraph.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:5000/graphql
elif [ "$2" = "grafbase_cache" ]; then
    ./grafbase -c grafbase_2_optimized.toml -s supergraph.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:5000/graphql
else
    echo "Error: invalid argument $2"
    exit 1
fi

