use crate::{
    config::Settings,
    generated::service::{calculator_client::CalculatorClient, AddRequest},
};
use tonic::Request;
use tracing::info;

pub async fn send_add_request(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect(settings.server.address.clone()).await?;

    let req = Request::new(AddRequest { a: 10, b: 5 });

    let response = client.add(req).await?;

    info!("Response from server: {}", response.into_inner().result);

    Ok(())
}
