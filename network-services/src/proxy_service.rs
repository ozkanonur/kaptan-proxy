use config_compiler::config::RoutesStruct;
use hyper::{Body, Client, Request, Response};
use std::{convert::Infallible, task::Poll};
use tower::Service;

#[derive(Clone)]
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
        let routes_ref = self.routes.as_ref();
        let mut target;

        let mut index = routes_ref.unwrap().iter().position(|r| {
            r.route == req.uri().to_string() || r.route.to_owned() + "/" == req.uri().to_string()
        });

        if index.is_some() {
            target = routes_ref.unwrap()[index.unwrap()].target.to_string();
        } else {
            index = routes_ref.unwrap().iter().position(|r| r.route == "/");
            // TODO: log error if no route exists
            target = routes_ref.unwrap()[index.unwrap()].target.to_string();
            target.push_str(&req.uri().to_string());
        }

        if target.chars().last() != Some('/') {
            target.push('/');
        }

        // TODO
        // Header manipulation on request
        // req.headers_mut().insert("Example-Header", "Here it is".parse().unwrap());

        Box::pin(async move {
            let client = Client::new();
            *req.uri_mut() = target.parse().unwrap();
            let res = client.request(req).await.unwrap();
            // TODO
            // Header manipulation on response
            // res.headers_mut().insert("Example-Header", "Here it is".parse().unwrap());
            Ok(res)
        })
    }
}
