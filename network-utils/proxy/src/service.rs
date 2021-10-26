use config_compiler::config::RoutesStruct;
use futures::Future;
use hyper::{header::HeaderName, Body, Client, Request, Response, StatusCode};
use std::{convert::Infallible, pin::Pin, task::Poll};
use tower::Service;

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
        // Routing
        let mut target;
        let mut index = self.routes.iter().position(|r| {
            r.inbound_route == req.uri().to_string()
                || r.inbound_route.to_owned() + "/" == req.uri().to_string()
        });

        if index.is_some() {
            target = self.routes[index.unwrap()].dest_addr.to_string();
        } else {
            index = self.routes.iter().position(|r| r.inbound_route == "/");

            // Return 404 if requested route doesn't exists
            if index.is_none() {
                return Box::pin(async {
                    let mut res = Response::new(Body::from(super::NOT_FOUND_BODY));

                    *res.status_mut() = StatusCode::NOT_FOUND;
                    res.headers_mut()
                        .insert("content-type", "application/json".parse().unwrap());

                    return Ok(res);
                });
            }

            target = self.routes[index.unwrap()].dest_addr.to_string();
            target.push_str(&req.uri().to_string());
        }

        if target.chars().last() != Some('/') {
            target.push('/');
        }
        let index = index.unwrap();

        // Request header manipulation
        self.routes[index]
            .req_headers
            .iter()
            .flatten()
            .for_each(|header| {
                if header.value.is_some() {
                    req.headers_mut().insert(
                        HeaderName::from_bytes(header.key.as_bytes()).unwrap(),
                        header.value.as_ref().unwrap().parse().unwrap(),
                    );
                } else {
                    req.headers_mut().remove(&header.key);
                }
            });

        let res_headers = self.routes[index].res_headers.clone();
        Box::pin(async move {
            let client = Client::new();
            *req.uri_mut() = target.parse().unwrap();
            let mut res = client.request(req).await.unwrap();

            // Response header manipulation
            res_headers.iter().flatten().for_each(|header| {
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
