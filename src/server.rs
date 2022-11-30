use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use metrics::{histogram, increment_counter, increment_gauge};
use metrics_exporter_prometheus::PrometheusBuilder;

use std::net::SocketAddr;
use std::time::Instant;

use tonic_health::server::{health_reporter, HealthReporter};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        let now = Instant::now();

        histogram!("foo_histgram", now.elapsed(), "api" => "echo", "result" => "ok");
        increment_counter!("foo_counter");
        increment_gauge!("foo_gauge", 10.0);

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = health_reporter();

    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    let metrics_addr: SocketAddr = "0.0.0.0:8081".parse()?;
    PrometheusBuilder::new()
        .with_http_listener(metrics_addr)
        .install()?;

    println!("HealthServer + GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
