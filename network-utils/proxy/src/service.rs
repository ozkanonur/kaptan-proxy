use config_compiler::config::RoutesStruct;
use futures::Future;
use hyper::{header::HeaderName, Body, Client, Request, Response, StatusCode};
use std::{convert::Infallible, pin::Pin, task::Poll};
use tower::Service;

use crate::router::RouterService;

#[derive(Clone)]
/// Middleware service that can route and proxy between
/// two connections.
///
/// (Runs after all the middlewares are executed.)
pub struct ProxyService {
    pub routes: Vec<RoutesStruct>,
}

impl Service<Request<Body>> for ProxyService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let route_dependencies = RouterService {}.get_dependencies(req.uri().to_string(), &self.routes);

        // Return 404 if route doesn't exists
        if route_dependencies.dest_addr.is_empty() {
            return Box::pin(async {
                let mut res = Response::new(Body::from(super::NOT_FOUND_BODY));

                *res.status_mut() = StatusCode::NOT_FOUND;
                res.headers_mut()
                    .insert("content-type", "application/json".parse().unwrap());

                return Ok(res);
            });
        }

        // Request header manipulation
        route_dependencies.req_headers.iter().flatten().for_each(|header| {
            if header.value.is_some() {
                req.headers_mut().insert(
                    HeaderName::from_bytes(header.key.as_bytes()).unwrap(),
                    header.value.as_ref().unwrap().parse().unwrap(),
                );
            } else {
                req.headers_mut().remove(&header.key);
            }
        });

        Box::pin(async move {
            let client = Client::new();
            *req.uri_mut() = route_dependencies.dest_addr.parse().unwrap();
            let mut res = client.request(req).await.unwrap();

            // Response header manipulation
            route_dependencies.res_headers.iter().flatten().for_each(|header| {
                if header.value.is_some() {
                    res.headers_mut().insert(
                        HeaderName::from_bytes(header.key.as_bytes()).unwrap(),
                        header.value.as_ref().unwrap().parse().unwrap(),
                    );
                } else {
                    res.headers_mut().remove(&header.key);
                }
            });

            Ok(res)
        })
    }
}
