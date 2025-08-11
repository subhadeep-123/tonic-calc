use std::{path, time::Duration};

use crate::{
    client::interceptor::AuthInterceptor,
    config::Settings,
    generated::service::{calculator_client::CalculatorClient, AddRequest, SubtractRequest},
};
use tonic::{
    service::interceptor::InterceptedService,
    transport::{Certificate, Channel, ClientTlsConfig, Endpoint},
    Request,
};
use tracing::{error, info};

pub struct GrpcClient {
    inner: CalculatorClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl GrpcClient {
    pub(crate) async fn tls_config(
        tls_path: String,
        domain_name: String,
    ) -> Result<ClientTlsConfig, Box<dyn std::error::Error>> {
        let data_dir = path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), &tls_path]);

        // Load CA certificate to verify the server's certificate
        let ca_cert = tokio::fs::read(data_dir.join("ca.crt")).await?;
        let ca_cert = Certificate::from_pem(ca_cert);

        let tls = ClientTlsConfig::new()
            .ca_certificate(ca_cert)
            .domain_name(domain_name); // must match SAN in server.crt

        Ok(tls)
    }

    pub async fn connect(settings: &Settings) -> Result<Self, Box<dyn std::error::Error>> {
        let token = &settings.auth.auth_token;
        if token.trim().is_empty() {
            return Err("Missing auth token in configuration".into());
        }

        let interceptor = AuthInterceptor::new(token)?;

        let tls_config =
            Self::tls_config(settings.tls.path.clone(), settings.tls.domain_name.clone()).await?;

        let address = format!("https://{}", &settings.client.address);
        info!("Connecting to server {} with TLS encryption", address);

        let endpoint = Endpoint::from_shared(address)?
            .timeout(Duration::from_secs(settings.client.timeout_secs))
            .tls_config(tls_config)?;

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
