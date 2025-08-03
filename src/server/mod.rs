use tonic::transport::Server;
use tonic_health::server::health_reporter;
use tracing::{error, info};

use crate::{
    config::Settings,
    generated::service::calculator_server::CalculatorServer,
    // middleware,
    server::{
        handler::CalculatorServiceImpl, health::monitor_health, signal::setup_shutdown_handler,
        state::AppState,
    },
};

pub mod handler;
pub mod health;
pub mod signal;
pub mod state;

pub async fn start_server(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let addr = settings.server.address.parse()?;
    info!("Server Starting on {}", addr);

    let state = AppState::new(settings.clone());

    let svc = CalculatorServiceImpl::new(state.clone());
    // let middleware = build_middleware_stack();

    // Health Check Service
    let (health_reporter, health_service) = health_reporter();
    monitor_health(health_reporter).await; // Start Monitoring

    let shutdown = setup_shutdown_handler();

    let server = Server::builder()
        // .layer(middleware)
        .add_service(health_service)
        .add_service(CalculatorServer::new(svc))
        .serve_with_shutdown(addr, async {
            let _ = shutdown.receiver.await;
            info!("Initiating graceful shutdown...");
        });

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        result = shutdown.task => {
            if let Err(e) = result {
                error!("Signal handling task error: {}", e);
            }
        }
    }

    info!("Server shutdown completed");
    Ok(())
}
