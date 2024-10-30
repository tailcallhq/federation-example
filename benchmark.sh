#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <json_file>"
  exit 1
fi

json_file="$1"

run_pair() {
  echo "Running configuration with file: $1 and JSON payload: $json_file"

  TC_LOG_LEVEL=error tailcall start "$1" &
  pid1=$!

  sleep 5

  hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D "$json_file" http://127.0.0.1:8030/graphql

  kill $pid1 2>/dev/null
}

run_pair ./configurations/1-basic.graphql
run_pair ./configurations/2-http-tweaks.graphql
run_pair ./configurations/3-http-cache.graphql
run_pair ./configurations/4-http-cache-directive.graphql
run_pair ./configurations/5-dedupe.graphql
run_pair ./configurations/6-all.graphql
