use tonic_micro::{client, config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = config::load()?;

    client::run(settings).await?;

    Ok(())
}
