extern crate futures;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::fmt::Error;

use futures::future::Future;
use hyper::{
    server::{Request, Response, Service},
    StatusCode::Ok,
};

struct Microservice;

impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    //process the request and response asynchronously
    fn call(&self, request: Request) -> Self::Future {
        info!("Microservice recieved a request: {:?}", request);
        Box::new(futures::future::ok(Response::new()))
    }
}

fn main() {
    //a logger configured via environment variables
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, || Ok(Microservice {}))
        .unwrap();
    server.run().unwrap();
}
