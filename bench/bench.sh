#! /bin/bash

set -eu

if [[ $# -ne 2 ]]
then
    echo "USAGE: bench.sh <host> <implementation>"
    exit 1
fi

HOST="$1"
IMPLEMENTATION="$2"

curl -X PUT \
  -H 'Content-Type: application/json' \
  "http://$HOST/api/values" \
  -d '{"key": "foo", "value": "bar"}'

RESULTS_PATH="$(pwd)/results"
mkdir -p "$RESULTS_PATH"

function run_bench {
  QPS_LIST=$1
  KEEP_ALIVE=$2

  RESULTS_FILE="$RESULTS_PATH/results-$IMPLEMENTATION-keepalive:$KEEP_ALIVE.csv"

  python3 process-results.py --print-headers > "$RESULTS_FILE"

  for qps in $QPS_LIST
  do
      fortio load \
        -qps $qps \
        -c 50 \
        -t 30s \
        -keepalive=$KEEP_ALIVE \
        -json "$RESULTS_PATH/result.json" \
        "http://$HOST/api/values/foo"

      python3 process-results.py --file "$RESULTS_PATH/result.json" --qps $qps >> "$RESULTS_FILE"
      rm -f "$RESULTS_PATH/result.json"
  done
}

run_bench "1000 2000 4000 8000 16000 24000 32000 48000 56000 64000" true
run_bench "1000 2000 3000 4000 6000 8000 10000 12000 16000" false
