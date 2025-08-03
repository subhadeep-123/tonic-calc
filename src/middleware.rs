// This file simply makes the `logging` module available to the rest of the crate.
pub mod logging;

use logging::LoggingMiddlewareLayer;
use tower::{Layer, ServiceBuilder};

/// Build the middleware stack used to wrap gRPC services.
/// You can add more layers (e.g., authentication, rate-limiting) here.
pub fn build_middleware_stack<S>() -> impl Layer<S> + Clone {
    ServiceBuilder::new().layer(LoggingMiddlewareLayer) // Add your logging layer
                                                        // .layer(OtherMiddlewareLayer) // You can chain more layers here
}
