FROM golang:1.13-alpine3.10 as builder
WORKDIR /go/src/github.com/betabandido/rest-api-perf/go/chi-rest-api
COPY main.go .
COPY handlers handlers
COPY vendor vendor
RUN go build -o /app/main main.go

FROM alpine:3.10
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY --from=builder /app .
CMD ["./main"]