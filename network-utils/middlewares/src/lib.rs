#![forbid(unsafe_code)]

use futures::future::Future;
use pin_project::pin_project;
use std::{pin::Pin, task::Poll};

pub mod logging_middleware;

#[pin_project]
/// Result type of middleware services.
///
/// Created to avoid of BoxFuture implementation which
/// causes runtime overhead, or Pinning the Result that causes
/// inability to use async blocks.
pub struct MiddlewareFuture<F> {
    #[pin]
    future: F,
}

impl<F> Future for MiddlewareFuture<F>
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
