use std::task::{Context, Poll};
use std::time::Instant;
use tonic::{Request, Status};
use tower::{Layer, Service};
use tracing::info;

// LoggingService is the actual middleware that wraps the inner gRPC service.
#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = tonic::Response<tonic::body::BoxBody>, Error = Status>,
    S::Future: Send + 'static,
    ReqBody: std::fmt::Debug + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        // Get gRPC path (like "/service.Calculator/Add")
        let method_path = req
            .metadata()
            .get(":path")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or("<unknown>".to_string());

        let start = Instant::now();
        info!("Incoming gRPC call: {}", method_path);

        let fut = self.inner.call(req);

        tokio::spawn(async move {
            let elapsed = start.elapsed();
            info!("Completed gRPC call {} in {:?}", method_path, elapsed);
        });

        fut
    }
}

// LoggingMiddlewareLayer is a factory for creating our middleware service.
// This is the struct we will use in src/server/mod.rs.
#[derive(Clone)]
pub struct LoggingMiddlewareLayer;

impl<S> Layer<S> for LoggingMiddlewareLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggingMiddleware { inner: service }
    }
}
