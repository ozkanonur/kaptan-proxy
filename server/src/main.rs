#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

use std::convert::Infallible;
use std::task::Poll;

use config_compiler::config::RoutesStruct;
use config_compiler::{config::Config, Compiler};

use hyper::{server::conn::Http, Body, Client, Request, Response};
use tokio::net::TcpListener;
use tower::{Service, ServiceBuilder};

mod runtime;

fn main() {
    let config = Config::read_from_fs();
    let routes = config.target.routes.clone();

    let listen_addr = format_args!("127.0.0.1:{}", config.runtime.inbound_port).to_string();
    runtime::create(&config.runtime).block_on(async move {
        let tcp_listener = TcpListener::bind(listen_addr).await.unwrap();
        let service_builder = ServiceBuilder::new();

        while let Ok((tcp_stream, _)) = tcp_listener.accept().await {
            let routes = routes.clone();
            let service_builder = service_builder.clone();

            tokio::spawn(async move {
                if let Err(http_err) = Http::new()
                    .http1_keep_alive(true)
                    .serve_connection(
                        tcp_stream,
                        service_builder.service(ProxyService { routes: &routes }),
                    )
                    .await
                {
                    eprintln!("Error while serving HTTP connection: {}", http_err);
                }
            });
        }
    });
}

struct ProxyService<'a> {
    routes: &'a Option<Vec<RoutesStruct>>,
}

impl Service<Request<Body>> for ProxyService<'_> {
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

        Box::pin(async move {
            let client = Client::new();
            *req.uri_mut() = target.parse().unwrap();
            let res = client.request(req).await.unwrap();
            Ok(res)
        })
    }
}

