use std::time::Duration;

use crate::{
    client::interceptor::AuthInterceptor,
    config::Settings,
    generated::service::{calculator_client::CalculatorClient, AddRequest, SubtractRequest},
};
use tonic::{
    service::interceptor::InterceptedService,
    transport::{Channel, Endpoint},
    Request,
};
use tracing::{error, info};

pub struct GrpcClient {
    inner: CalculatorClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl GrpcClient {
    pub async fn connect(settings: &Settings) -> Result<Self, Box<dyn std::error::Error>> {
        let token = &settings.auth.auth_token;
        if token.trim().is_empty() {
            return Err("Missing auth token in configuration".into());
        }

        let interceptor = AuthInterceptor::new(token)?;

        let endpoint =
            Endpoint::from_shared(settings.server.address.clone())?.timeout(Duration::from_secs(5));

        let channel = endpoint.connect().await.map_err(|e| {
            error!(
                "Failed to connect to gRPC server at {}: {:?}",
                settings.server.address, e
            );
            e
        })?;

        let client = CalculatorClient::with_interceptor(channel, interceptor);

        Ok(Self { inner: client })
    }

    pub async fn add(&mut self, a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let request = Request::new(AddRequest { a, b });

        let response = self.inner.add(request).await;

        match response {
            Ok(res) => {
                let result = res.into_inner().result;
                info!(%result, a, b, "Add operation succeeded");
                Ok(result)
            }
            Err(err) => {
                error!("Add RPC failed: {:?}", err);
                Err(err.into())
            }
        }
    }

    pub async fn subtract(&mut self, a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let request = Request::new(SubtractRequest { a, b });

        let response = self.inner.subtract(request).await;

        match response {
            Ok(res) => {
                let result = res.into_inner().result;
                info!(%result, a, b, "Subtraction operation succeeded");
                Ok(result)
            }
            Err(err) => {
                error!("Add RPC failed: {:?}", err);
                Err(err.into())
            }
        }
    }
}
