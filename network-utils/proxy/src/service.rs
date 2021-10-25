use config_compiler::config::RoutesStruct;
use hyper::{header::HeaderName, Body, Client, Request, Response};
use std::{convert::Infallible, task::Poll};
use tower::Service;

#[derive(Clone)]
/// Middleware service that can route and proxy between
/// two connections.
///
/// (Runs after all the middlewares are executed.)
pub struct ProxyService {
    pub routes: Option<Vec<RoutesStruct>>,
}

impl Service<Request<Body>> for ProxyService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let routes = self.routes.clone();

        // Routing
        let mut target;
        let mut index = routes.as_ref().unwrap().iter().position(|r| {
            r.inbound_route == req.uri().to_string() || r.inbound_route.to_owned() + "/" == req.uri().to_string()
        });

        if index.is_some() {
            target = routes.as_ref().unwrap()[index.unwrap()].dest_addr.to_string();
        } else {
            index = routes.as_ref().unwrap().iter().position(|r| r.inbound_route == "/");
            // TODO: log error if no route exists
            target = routes.as_ref().unwrap()[index.unwrap()].dest_addr.to_string();
            target.push_str(&req.uri().to_string());
        }

        if target.chars().last() != Some('/') {
            target.push('/');
        }

        // Request header manipulation
        routes.as_ref().unwrap()[index.unwrap()]
            .req_headers
            .iter()
            .flatten()
            .for_each(|header| {
                req.headers_mut().insert(
                    HeaderName::from_bytes(header.key.as_bytes()).unwrap(),
                    header.value.parse().unwrap(),
                );
            });

        Box::pin(async move {
            let client = Client::new();
            *req.uri_mut() = target.parse().unwrap();
            let mut res = client.request(req).await.unwrap();

            // Response header manipulation
            routes.as_ref().unwrap()[index.unwrap()]
                .res_headers
                .iter()
                .flatten()
                .for_each(|header| {
                    res.headers_mut().insert(
                        HeaderName::from_bytes(header.key.as_bytes()).unwrap(),
                        header.value.parse().unwrap(),
                    );
                });

            Ok(res)
        })
    }
}
