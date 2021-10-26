#![forbid(unsafe_code)]

pub use hyper::server::conn::Http;
pub use tower::ServiceBuilder;

pub mod service;

/// Serialized json schema of 404 pages
const NOT_FOUND_BODY: &str =
    "{ \"description\": \"The requested route could not be found.\", \"from\": \"kaptan\" }";
