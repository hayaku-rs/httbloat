extern crate log;
extern crate env_logger;
extern crate futures;
extern crate tokio_proto;
extern crate tokio_service;
extern crate httbloat;

use futures::future;
use tokio_proto::TcpServer;
use tokio_service::Service;
use httbloat::{Request, Response, Http};

use std::io;

struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&mut self, _req: Request) -> Self::Future {
        let mut resp = Response::new();
        resp.body(b"Hello, world!").unwrap();
        future::ok(resp)
    }
}

fn main() {
    env_logger::init().unwrap();
    let addr = "0.0.0.0:3000".parse().unwrap();
    let mut srv = TcpServer::new(Http, addr);
    srv.threads(4);
    srv.serve(|| Ok(HelloWorld));
}
