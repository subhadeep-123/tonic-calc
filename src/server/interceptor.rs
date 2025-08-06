use tonic::{
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    Request, Status,
};
use tracing::{debug, warn};

#[derive(Clone)]
pub struct AuthValidator {
    token: MetadataValue<Ascii>,
}

impl AuthValidator {
    // Create new AuthValidator with a bearer token
    pub fn new(secret: &str) -> Result<Self, tonic::Status> {
        Ok(Self {
            token: format!("Bearer {}", secret)
                .parse()
                .map_err(|_| Status::internal("Failed to parse auth token"))?, // Validate token adn return
        })
    }
}

impl Interceptor for AuthValidator {
    fn call(&mut self, req: Request<()>) -> Result<Request<()>, Status> {
        match req.metadata().get("authorization") {
            Some(t) if &self.token == t => {
                debug!("Authorization succeeded");
                Ok(req)
            }
            _ => {
                warn!("Authorization failed for request: {:?}", req.metadata());
                Err(Status::unauthenticated("No valid auth token"))
            }
        }
    }
}
