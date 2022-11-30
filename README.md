# rust_tonic

## Tuto
https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

## Prepare
If you've never run it.
```
brew install protobuf
```

## Run
```sh
cargo run --bin helloworld-server
```
## Usage
If you've never run it.
```
brew install grpcurl
```
### request
```sh
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::]:50051' helloworld.Greeter/SayHello
```
### heakth check
```sh
grpcurl -plaintext -d '{"service": "mygrpc"}' [::]:50051 grpc.health.v1.Health.Check

grpcurl -plaintext [::]:50051 grpc.health.v1.Health.Check
```

## Appendix
### client
```sh
cargo run --bin helloworld-client
```
### metrics
```sh
curl -i http://localhost:8081/
```
