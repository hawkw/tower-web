use error::Catch;
use routing::{Resource, RoutedService};
use util::{
    http::{HttpMiddleware, HttpService},
    BufStream,
};

use futures::Poll;
use http;
use tower_service::Service;

use std::fmt;

/// The service defined by `ServiceBuilder`.
///
/// `WebService` contains the resources, routes, middleware, catch handlers, ...
/// that were defined by the builder. It implements `tower_service::Service`,
/// which exposes an HTTP request / response API.
pub struct WebService<T, U, M, B>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>, B>,
    B: BufStream,
{
    /// The routed service wrapped with middleware
    inner: M::Service,
}

impl<T, U, M, B> WebService<T, U, M, B>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>, B>,
    B: BufStream,
{
    pub(crate) fn new(inner: M::Service) -> WebService<T, U, M, B> {
        WebService { inner }
    }
}

impl<T, U, M, B> Service<http::Request<B>> for WebService<T, U, M, B>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>, B>,
    B: BufStream,
{
    type Response = http::Response<M::ResponseBody>;
    type Error = M::Error;
    type Future = <M::Service as HttpService<B>>::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_http_ready()
    }

    fn call(&mut self, request: http::Request<B>) -> Self::Future {
        self.inner.call_http(request)
    }
}

impl<T, U, M, B> fmt::Debug for WebService<T, U, M, B>
where T: Resource + fmt::Debug,
      U: Catch + fmt::Debug,
      M: HttpMiddleware<RoutedService<T, U>, B> + fmt::Debug,
      M::Service: fmt::Debug,
      B: BufStream + fmt::Debug,
      M::ResponseBody: fmt::Debug,
      M::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("WebService")
            .field("inner", &self.inner)
            .finish()
    }
}
