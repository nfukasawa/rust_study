extern crate hyper;
extern crate path_tree;

use hyper::http::Error;
use hyper::Method;
type Request = hyper::Request<hyper::Body>;
type Response = hyper::Response<hyper::Body>;
use path_tree::PathTree;

use super::context::Context;

type HandlerCallback = Box<dyn Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync>;

pub struct Router {
    routes: PathTree<HandlerCallback>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: PathTree::new(),
        }
    }

    pub fn get<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync + 'static,
    {
        self.request(&Method::GET, path, handler)
    }

    pub fn head<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync + 'static,
    {
        self.request(&Method::HEAD, path, handler)
    }

    pub fn post<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync + 'static,
    {
        self.request(&Method::POST, path, handler)
    }

    pub fn put<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync + 'static,
    {
        self.request(&Method::PUT, path, handler)
    }

    pub fn patch<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync + 'static,
    {
        self.request(&Method::PATCH, path, handler)
    }

    pub fn delete<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync + 'static,
    {
        self.request(&Method::DELETE, path, handler)
    }

    pub fn options<'a, S, H>(&mut self, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync + 'static,
    {
        self.request(&Method::OPTIONS, path, handler)
    }

    pub fn request<'a, S, H>(&mut self, method: &Method, path: S, handler: H) -> &mut Self
    where
        S: Into<&'a str>,
        H: Fn(&Context, &Request) -> Result<Response, Error> + Send + Sync + 'static,
    {
        self.routes
            .insert(&tree_path(method, path.into()), Box::new(handler));
        self
    }

    pub fn exec(&self, req: &Request) -> Result<Response, Error> {
        match self.routes.find(&tree_path(req.method(), req.uri().path())) {
            Some((handler, params)) => {
                let mut ctx = Context::new();
                ctx.set_params(params);
                handler(&ctx, &req)
            }
            None => hyper::Response::builder()
                .status(hyper::StatusCode::NOT_FOUND)
                .body(hyper::Body::from("Not Found")),
        }
    }
}

#[inline]
fn tree_path(method: &Method, path: &str) -> String {
    format!("/{}{}", method, path)
}
