use super::router::Router;
use hyper::rt::Future;
use hyper::service::service_fn;
use std::sync::Arc;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serve(self, router: Router, port: u16) {
        let router = Arc::new(router);

        let svc = move || {
            let router = router.clone();
            service_fn(move |req| router.exec(&req))
        };
        let server = hyper::Server::bind(&([127, 0, 0, 1], port).into())
            .serve(svc)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }
}
