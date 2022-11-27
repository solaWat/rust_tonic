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
```sh
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::]:50051' helloworld.Greeter/SayHello
```

## Appendix
```sh
cargo run --bin helloworld-client
```
```sh
curl http://localhost:9000/
```
