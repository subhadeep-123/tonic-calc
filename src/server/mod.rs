use std::net::ToSocketAddrs;
use std::{error, fs, path};

use tonic::transport::Identity;
use tonic::transport::Server;
use tonic::transport::ServerTlsConfig;
use tonic_health::server::health_reporter;
use tracing::{error, info};

use crate::{
    config::Settings,
    generated::service::calculator_server::CalculatorServer,
    server::{
        handler::CalculatorServiceImpl, health::monitor_health, interceptor::AuthValidator,
        signal::setup_shutdown_handler, state::AppState,
    },
};

pub mod handler;
pub mod health;
pub mod interceptor;
pub mod signal;
pub mod state;

async fn tls_identity(tls_path: String) -> Result<Identity, Box<dyn error::Error>> {
    let data_dir = path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), &tls_path]);

    let cert = fs::read_to_string(data_dir.join("server.crt"))?;

    let key = fs::read_to_string(data_dir.join("server.key"))?;

    let identity = Identity::from_pem(cert, key);

    Ok(identity)
}

pub async fn start_server(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let addr = settings
        .server
        .address
        .to_socket_addrs()?
        .next()
        .ok_or("Invalid address")?;
    info!("Server Starting on {} with TLS Encryption", addr);

    let state = AppState::new(settings.clone());

    let svc = CalculatorServiceImpl::new(state.clone());

    // Health Check Service
    let (health_reporter, health_service) = health_reporter();
    monitor_health(health_reporter).await; // Start Monitoring

    // Handle graceful shutdown
    let shutdown = setup_shutdown_handler();

    // Interceptor Config
    let interceptor = AuthValidator::new(&settings.auth.auth_token)?;

    // TLS Identity
    let identity = tls_identity(String::from(&settings.tls.path)).await?;

    let server = Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(health_service)
        .add_service(CalculatorServer::with_interceptor(svc, interceptor))
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
