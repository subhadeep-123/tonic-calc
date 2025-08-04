use tonic_micro::{client, config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = config::load()?;

    settings.server.address = format!("http://{}", settings.server.address);

    client::run(settings).await?;

    Ok(())
}
