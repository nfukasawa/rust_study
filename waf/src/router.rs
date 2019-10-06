extern crate hyper;
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use std::sync::Arc;

use hyper::Method;
type Request = hyper::Request<hyper::Body>;
type Response = hyper::Response<hyper::Body>;

use super::context::Context;
use super::path::Path;

type HandlerCallback = Box<dyn Fn(&Context, &Request) -> Response + Send + Sync>;

pub struct Router {
    routes: Vec<(Method, Path, HandlerCallback)>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn get<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Response + Send + Sync + 'static,
    {
        self.request(Method::GET, path, handler)
    }

    pub fn head<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Response + Send + Sync + 'static,
    {
        self.request(Method::HEAD, path, handler)
    }

    pub fn post<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Response + Send + Sync + 'static,
    {
        self.request(Method::POST, path, handler)
    }

    pub fn put<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Response + Send + Sync + 'static,
    {
        self.request(Method::PUT, path, handler)
    }

    pub fn patch<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Response + Send + Sync + 'static,
    {
        self.request(Method::PATCH, path, handler)
    }

    pub fn delete<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Response + Send + Sync + 'static,
    {
        self.request(Method::DELETE, path, handler)
    }

    pub fn options<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Response + Send + Sync + 'static,
    {
        self.request(Method::OPTIONS, path, handler)
    }

    pub fn request<'a, S, H>(&mut self, method: Method, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Response + Send + Sync + 'static,
    {
        self.routes
            .push((method, Path::new(path.into()), Box::new(handler)));
        self
    }

    pub fn run(self, port: u16) {
        let routes = Arc::new(self.routes);
        let svc = move || {
            let routes = routes.clone();
            service_fn_ok(move |req| {
                for (method, path, handler) in routes.iter() {
                    if method == req.method() {
                        let (ok, params) = path.matches(req.uri().path());
                        if ok {
                            return handler(&Context::new(params), &req);
                        }
                    }
                }

                hyper::Response::builder()
                    .status(hyper::StatusCode::NOT_FOUND)
                    .body(hyper::Body::from("Not Found"))
                    .unwrap()
            })
        };
        let server = hyper::Server::bind(&([127, 0, 0, 1], port).into())
            .serve(svc)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }
}
