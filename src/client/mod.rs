mod grpc;

use crate::{client::grpc::send_add_request, config::Settings};

pub async fn run(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    send_add_request(settings).await
}
