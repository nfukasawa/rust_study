extern crate hyper;

#[macro_use]
extern crate serde_json;

use hyper::{Body, Response, StatusCode};
use waf::{Router, Server};

fn main() {
    let mut router = Router::new();
    router.get("/json", |_, _| {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"message": "Hello, World!"}).to_string()))
    });

    Server::new().serve(router, 3000);
}
