extern crate hyper;

#[macro_use]
extern crate serde_json;

use hyper::{Body, Response, StatusCode};
use std::sync::{Arc, Mutex};
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
        })
        .get("/counter", move |_, _| {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"counter": *counter}).to_string()))
        })
        .get("/foo/:id", |ctx, _| {
            let id = ctx.param("id");
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "id": id }).to_string()))
        })
        .middleware("/users/*", |ctx, req| {
            match req.headers().get("Authorization") {
                Some(val) => {
                    let val = val.to_str().unwrap().to_string();
                    if val.starts_with("Bearer ") {
                        ctx.set_value("username", Box::new("John".to_string()));
                    }
                }
                None => (),
            }
            Ok(())
        })
        .get("/users/:id", |ctx, _| {
            let username = match ctx.value("username") {
                Some(val) => match val.downcast_ref::<String>() {
                    // TODO: not matched. why?
                    Some(val) => val,
                    None => "unknown",
                },
                None => "unknown",
            };
            let id = ctx.param("id");

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({ "id": id, "username": username }).to_string(),
                ))
        });

    Server::new().serve(router, 3000);
}
