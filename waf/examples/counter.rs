extern crate hyper;

#[macro_use]
extern crate serde_json;

use hyper::{Body, Response, StatusCode};
use std::sync::{Arc, Mutex};
use waf::{Router, Server};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut router = Router::new();
    router.get("/counter", move |_, _| {
        let mut counter = counter.lock().unwrap();
        *counter += 1;
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"counter": *counter}).to_string()))
    });

    Server::new().serve(router, 3000);
}
