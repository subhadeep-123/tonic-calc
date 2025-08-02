use tonic::transport::Server;

use crate::{
    config::Settings,
    generated::service::calculator_server::CalculatorServer,
    // middleware,
    server::{handler::CalculatorServiceImpl, state::AppState},
};

pub mod handler;
pub mod state;

pub async fn start_server(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let addr = settings.server.address.parse()?;
    let state = AppState::new(settings.clone());

    let svc = CalculatorServiceImpl::new(state.clone());
    // let middleware = build_middleware_stack();

    Server::builder()
        // .layer(middleware)
        .add_service(CalculatorServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
