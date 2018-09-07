use super::Middleware;

use tower_service::Service;

/// Two middlewares chained together.
///
/// This type is produced by `Middleware::chain`.
#[derive(Debug)]
pub struct Chain<Inner, Outer>
{
    inner: Inner,
    outer: Outer,
}

impl<Inner, Outer> Chain<Inner, Outer> {
    /// Create a new `Chain`.
    pub fn new(inner: Inner, outer: Outer) -> Self {
        Chain {
            inner,
            outer,
        }
    }
}

impl<S, Inner, Outer, R> Middleware<S, R> for Chain<Inner, Outer>
where S: Service<R>,
      Inner: Middleware<S, R>,
      Outer: Middleware<Inner::Service, R>,
{
    type Response = Outer::Response;
    type Error = Outer::Error;
    type Service = Outer::Service;

    fn wrap(&self, service: S) -> Self::Service {
        self.outer.wrap(
            self.inner.wrap(service))
    }
}

impl<T, Inner, Outer> ::util::Chain<T> for Chain<Inner, Outer> {
    type Output = Chain<Self, T>;

    fn chain(self, other: T) -> Self::Output {
        Chain::new(self, other)
    }
}
