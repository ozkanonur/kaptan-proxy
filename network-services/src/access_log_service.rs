use futures::future::Future;
use hyper::Request;
use pin_project::pin_project;
use std::{pin::Pin, task::Poll};
use tower::Service;

#[pin_project]
/// Result type of tower implementation of AccessLogService.
///
/// Created to avoid of BoxFuture implementation which
/// causes runtime overhead, or Pinning the Result that causes
/// inability to use async blocks.
pub struct LoggingFuture<F> {
    #[pin]
    future: F,
}

impl<F> Future for LoggingFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res = match this.future.poll(cx) {
            Poll::Ready(res) => res,
            Poll::Pending => return Poll::Pending,
        };

        Poll::Ready(res)
    }
}

/// Middleware service that can route and proxy between
/// two connections.
///
/// (Runs after all the middlewares are executed.)
pub struct AccessLogService<S> {
    inner: S,
}

impl<S> AccessLogService<S> {
    /// Creates and returns an instance of AccessLogService.
    ///
    /// Takes another middleware as an argument.
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for AccessLogService<S>
where
    S: 'static + Service<Request<B>> + Clone + Send,
    B: 'static + Send,
    S::Future: 'static + Send
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = LoggingFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        // TODO
        // Log to file
        // println!("Logger middleware triggered");
        LoggingFuture {
            future: self.inner.call(req),
        }
    }
}

