FROM golang:1.12.5-alpine3.9 as builder
WORKDIR /go/src/github.com/betabandido/rest-api-perf/go/gorilla-rest-api
COPY main.go .
COPY handlers handlers
COPY vendor vendor
RUN go build -o /app/main main.go

FROM alpine:3.9
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY --from=builder /app .
CMD ["./main"]