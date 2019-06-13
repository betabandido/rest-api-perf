go run main.go

curl -X PUT \
  -H 'Content-Type: application/json' \
  http://localhost:8000/api/values \
  -d '{"key": "foo", "value": "bar"}'

wrk -c 4 -d 30 -t 4 --latency http://localhost:8000/api/values/foo

## Initial results

```
Running 30s test @ http://localhost:8000/api/values/foo
  4 threads and 4 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   150.94us  647.77us  39.00ms   99.02%
    Req/Sec     8.62k   807.28    11.12k    87.94%
  Latency Distribution
     50%  101.00us
     75%  119.00us
     90%  156.00us
     99%  778.00us
  1031141 requests in 30.10s, 147.51MB read
Requests/sec:  34258.18
Transfer/sec:      4.90MB
```
