use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use log::LevelFilter;
use num_cpus;
use simple_logger::SimpleLogger;

use std::collections::HashMap;
use std::thread;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use actix_web_tonic;

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

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_colors(true)
        .with_utc_timestamps()
        .with_level(LevelFilter::Info)
        .init()
        .expect("oops, failed to initialize SimpleLogger.");

    let actix_worker_threads = (num_cpus::get_physical() * 1 / 4).max(1);
    let tonic_worker_threads = (num_cpus::get_physical() - actix_worker_threads).max(1);
    log::info!(
        "actix_worker_threads={actix_worker_threads}, tonic_worker_threads={tonic_worker_threads}"
    );

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    let tonic_future = {
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
    };

    let actix_future = {
        let mut labels = HashMap::new();
        labels.insert("application".to_string(), "solawat".to_string());
        let prometheus = PrometheusMetricsBuilder::new("api")
            .endpoint("/metrics")
            .const_labels(labels)
            .build()
            .unwrap();

        HttpServer::new(move || {
            App::new()
                .wrap(prometheus.clone())
                .service(web::resource("/health").to(health))
        })
        .bind("[::1]:8080")
        .unwrap()
        .workers(actix_worker_threads)
        .run()
    };

    // run in concurrent both of actix-web and tonic on the single bin process.
    thread::spawn(move || {
        let r = actix_web_tonic::invoke(actix_future, tonic_future, tonic_worker_threads);
        log::info!("The app will be exit; The result of actix_web_tonic::invoke is ... {r:?}");
    })
    .join()
    .expect("Thread panicked");

    Ok(())
}
