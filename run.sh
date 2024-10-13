#!/bin/bash

# free the ports at the end
function cleanup {
    echo "Killing existing processes on ports 8071, 8082, 8080, 8081, 8083"
    lsof -ti:8071,8082,8080,8081,8083 | xargs kill -9
}

# Set up trap for various signals
trap cleanup SIGINT SIGTERM EXIT

paths=("employees" "family" "hobbies" "mood" "products")

# Build and run each project
for path in "${paths[@]}"; do
    echo "Building and running $path"
    (cd "$path" && cargo run --release --quiet) &
done

# Wait for all background processes to finish
wait
