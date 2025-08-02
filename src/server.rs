pub mod service {
    tonic::include_proto!("service");
}

use service::{
    AddRequest, AddResponse, SubtractRequest, SubtractResponse,
    calculator_server::{Calculator, CalculatorServer},
};
use tonic::{Request, Response, Status, transport::Server};
use tracing::info;

// Implement the service
#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let req = request.into_inner();
        let result = req.a + req.b;

        println!("Processing add request: {} + {} = {}", req.a, req.b, result);

        Ok(Response::new(AddResponse { result }))
    }

    async fn subtract(
        &self,
        request: Request<SubtractRequest>,
    ) -> Result<Response<SubtractResponse>, Status> {
        let req = request.into_inner();
        let result = req.a - req.b;

        println!(
            "Processing subtract request: {} - {} = {}",
            req.a, req.b, result
        );

        Ok(Response::new(SubtractResponse { result }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let addr = "[::1]:50051".parse()?;
    let service = CalculatorService::default();

    info!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(CalculatorServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
