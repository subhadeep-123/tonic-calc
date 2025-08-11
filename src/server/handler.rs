use crate::server::state::AppState;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::generated::service::{
    calculator_server::Calculator, AddRequest, AddResponse, SubtractRequest, SubtractResponse,
};

// Implement the service
#[derive(Debug)]
pub struct CalculatorServiceImpl {
    state: Arc<AppState>,
}

impl CalculatorServiceImpl {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorServiceImpl {
    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let _ = &self.state;

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
