dotnet run --configuration Release --urls=http://*:8000

docker run -it --rm -p 8000:80 -e REST_API_PERF_REPO=in-memory rest-api-perf-csharp

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
    Latency   300.74us  383.53us  22.00ms   97.60%
    Req/Sec     3.59k   451.16     4.21k    73.83%
  Latency Distribution
     50%  249.00us
     75%  288.00us
     90%  377.00us
     99%    1.08ms
  428821 requests in 30.00s, 76.07MB read
Requests/sec:  14292.54
Transfer/sec:      2.54MB
```
