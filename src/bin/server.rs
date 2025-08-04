use tonic_micro::{
    config::{load, Settings},
    server::start_server,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let settings: Settings = load()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    start_server(settings).await?;

    Ok(())
}
