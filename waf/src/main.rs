extern crate hyper;

#[macro_use]
extern crate serde_json;

use hyper::{Body, Response, StatusCode};
use waf::Router;

fn main() {
    let mut r = Router::new();
    r.get("/json", |_, _| {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"message": "Hello, World!"}).to_string()))
            .unwrap()
    });
    r.get("/foo/:id", |ctx, _| {
        let id = ctx.param("id");
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({ "id": id }).to_string()))
            .unwrap()
    });
    r.run(3000);
}
