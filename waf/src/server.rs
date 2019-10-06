use super::router::{do_routing, get_routes, Router};
use hyper::rt::Future;
use hyper::service::service_fn;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serve(self, router: Router, port: u16) {
        let routes = get_routes(router);

        let svc = move || {
            let routes = routes.clone();
            service_fn(move |req| do_routing(&routes, &req))
        };
        let server = hyper::Server::bind(&([127, 0, 0, 1], port).into())
            .serve(svc)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }
}
