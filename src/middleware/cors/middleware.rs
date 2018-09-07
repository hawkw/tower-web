use super::{Config, CorsService};
use middleware::Middleware;

use http;
use util::{BufStream, http::HttpService};

use std::sync::Arc;

#[derive(Debug)]
pub struct CorsMiddleware {
    config: Arc<Config>,
}

impl CorsMiddleware {
    pub(super) fn new(config: Config) -> CorsMiddleware {
        let config = Arc::new(config);
        CorsMiddleware { config }
    }
}

impl<S, R> Middleware<S, http::Request<R>> for CorsMiddleware
where
    S: HttpService<R>,
    R: BufStream,
{
    type Response = http::Response<Option<S::ResponseBody>>;
    type Error = S::Error;
    type Service = CorsService<S>;

    fn wrap(&self, service: S) -> Self::Service {
        CorsService::new(service, self.config.clone())
    }
}
