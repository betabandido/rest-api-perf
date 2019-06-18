#! /bin/bash

if [[ -z "$1" ]]
then
    echo "USAGE: bench.sh <host>"
    exit 1
fi

HOST="$1"

curl -X PUT \
  -H 'Content-Type: application/json' \
  "http://$HOST/api/values" \
  -d '{"key": "foo", "value": "bar"}'

python3 process-results.py --print-headers > results.csv

for qps in 100 200 400 800 1600 2400 3200 4800 6400 9600 12800 16000 19200 22400 25600 32000
do
    fortio load -qps $qps -c 64 -t 10s -json result.json "http://$HOST/api/values/foo"
    python3 process-results.py --file result.json --qps $qps >> results.csv
done
