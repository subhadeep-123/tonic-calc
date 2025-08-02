use tonic::transport::Server;
use tonic_health::server::health_reporter;

use crate::{
    config::Settings,
    generated::service::calculator_server::CalculatorServer,
    // middleware,
    server::{handler::CalculatorServiceImpl, health::monitor_health, state::AppState},
};

pub mod handler;
pub mod health;
pub mod state;

pub async fn start_server(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let addr = settings.server.address.parse()?;
    let state = AppState::new(settings.clone());

    let svc = CalculatorServiceImpl::new(state.clone());
    // let middleware = build_middleware_stack();

    // Health Check Service
    let (health_reporter, health_service) = health_reporter();
    monitor_health(health_reporter).await; // Start Monitoring

    Server::builder()
        // .layer(middleware)
        .add_service(health_service)
        .add_service(CalculatorServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
