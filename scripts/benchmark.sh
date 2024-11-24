#!/bin/bash

benchmark_file="bench-hey-$1.json"

# Define a function to kill processes running on a specific port
kill_port_process() {
    local port=$1
    pid=$(lsof -ti:$port)
    if [ -n "$pid" ]; then
        kill -9 "$pid"
    fi
}
cleanup() {
    # List of processes to terminate
    processes=("tailcall" "nginx" "wunder" "apollo" "grafbase")

    for process in "${processes[@]}"; do
        for pid in $(ls /proc | grep -E '^[0-9]+$' | xargs -I {} sh -c "grep -l '$process' /proc/{}/comm 2>/dev/null" | sed 's|/proc/||g' | sed 's|/comm||g'); do
            kill -9 "$pid" 2>/dev/null || true
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


if [ "$2" = "source_graphql" ]; then
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:$source_port/graphql
elif [ "$2" = "source_rest_api" ]; then
    hey -n 200 -z 10s -m GET -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:$source_port/$1-json
elif [ "$2" = "nginx_graphql" ]; then
    nginx
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:$nginx_port/graphql
elif [ "$2" = "tailcall_default" ]; then
    TC_LOG_LEVEL=error tailcall start configurations/tailcall/default/1-basic.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:8030/graphql
elif [ "$2" = "tailcall_full_conf" ]; then
    TC_LOG_LEVEL=error tailcall start configurations/tailcall/cached/6-all.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:8030/graphql
elif [ "$2" = "wundergraph_no_opt" ]; then
    ./wunder --config configurations/wundergraph/wundergraph_1_basic.json > /dev/null &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:3002/graphql
elif [ "$2" = "wundergraph_default" ]; then
    ./wunder --config configurations/wundergraph/wundergraph_3_cached.json > /dev/null &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:3002/graphql
elif [ "$2" = "apollo_router" ]; then
    ./apollo -s configurations/supergraph.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:4000
elif [ "$2" = "grafbase_default" ]; then
    ./grafbase -c configurations/grafbase/grafbase_1_basic.toml -s configurations/supergraph.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:5000/graphql
elif [ "$2" = "grafbase_cache" ]; then
    ./grafbase -c configurations/grafbase/grafbase_2_optimized.toml -s configurations/supergraph.graphql &
    sleep 5
    hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "bench-hey-$1.json" http://127.0.0.1:5000/graphql
else
    echo "Error: invalid argument $2"
    exit 1
fi

