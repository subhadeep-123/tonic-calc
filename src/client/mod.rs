mod grpc;
mod interceptor;

use crate::{client::grpc::GrpcClient, config::Settings};

pub async fn run(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GrpcClient::connect(&settings).await?;
    client.add(10, 20).await?;
    client.subtract(45, 20).await?;
    Ok(())
}
