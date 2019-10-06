extern crate hyper;

#[macro_use]
extern crate serde_json;

use std::sync::{Arc, Mutex};
use hyper::{Body, Response, StatusCode};
use waf::{Router, Server};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut router = Router::new();
    router
        .get("/json", |_, _| {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"message": "Hello, World!"}).to_string()))
                .unwrap()
        })
        .get("/counter", move |_, _| {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"counter": *counter}).to_string()))
                .unwrap()
        })
        .get("/foo/:id", |ctx, _| {
            let id = ctx.param("id");
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "id": id }).to_string()))
                .unwrap()
        });

    Server::new().serve(router, 3000);
}
