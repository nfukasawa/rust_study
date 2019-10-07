extern crate hyper;

#[macro_use]
extern crate serde_json;

use hyper::{Body, Response, StatusCode};
use waf::{Router, Server};

fn main() {
    let mut router = Router::new();
    router
        .middleware("/users/*", |ctx, req| {
            match req.headers().get("Authorization") {
                Some(val) => {
                    let val = val.to_str().unwrap().to_string();
                    if val.starts_with("Bearer ") {
                        ctx.set_value("username", "John".to_string());
                    }
                }
                None => (),
            }
            Ok(())
        })
        .get("/users/:id", |ctx, _| {
            let unknown = "unknown".to_string();
            let username: &String = match ctx.value("username") {
                Some(val) => val,
                None => &unknown,
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
