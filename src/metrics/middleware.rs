use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::dev::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};

use std::time::Instant;

pub struct Metrics;

use crate::metrics;

impl Metrics {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S> for Metrics
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MetricsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MetricsMiddleware { service })
    }
}

pub struct MetricsMiddleware<S> {
    service: S,
}

impl<S, B> Service for MetricsMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let timer = Instant::now();
        let route = req.match_pattern().unwrap_or(req.uri().path().to_owned()).to_owned();
        let method = req.method().as_str().to_owned();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let status = res.status().as_u16();

            metrics::N_REQUESTS
                .with_label_values(&[
                    &method,
                    &route,
                    &status.to_string(),
                    &metrics::INSTANCE_ID,
                    &metrics::INDEX_ID,
                    &metrics::ORGANIZATION_ID,
                ])
                .inc();

            metrics::REQUEST_TIME_HISTOGRAM
                .with_label_values(&[
                    &method,
                    &route,
                    &status.to_string(),
                    &metrics::INSTANCE_ID,
                    &metrics::INDEX_ID,
                    &metrics::ORGANIZATION_ID,
                ])
                .observe(timer.elapsed().as_secs_f64());

            Ok(res)
        })
    }
}
