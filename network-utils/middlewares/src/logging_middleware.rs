use hyper::Request;
use logger::{access_log::AccessLog, LogCapabilities};
use std::task::Poll;
use tower::Service;

use crate::MiddlewareFuture;

/// Middleware service that can route and proxy between
/// two connections.
///
/// (Runs after all the middlewares are executed.)
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> LoggingMiddleware<S> {
    /// Creates and returns an instance of LoggingMiddleware.
    ///
    /// Takes another middleware as an argument.
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for LoggingMiddleware<S>
where
    S: 'static + Service<Request<B>> + Clone + Send,
    B: 'static + Send + std::fmt::Debug,
    S::Future: 'static + Send,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = MiddlewareFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        AccessLog {
            method: &req.method(),
            uri: &req.uri(),
            version: &req.version(),
            headers: &req.headers(),
        }
        .write();

        MiddlewareFuture {
            future: self.inner.call(req),
        }
    }
}

