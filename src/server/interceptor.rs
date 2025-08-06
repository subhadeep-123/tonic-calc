use tonic::{
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    Request, Status,
};

#[derive(Clone)]
pub struct AuthInterceptor {
    token: MetadataValue<Ascii>,
}

impl AuthInterceptor {
    pub fn new(secret: &str) -> Self {
        Self {
            token: format!("Bearer {}", secret).parse().unwrap(),
        }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, req: Request<()>) -> Result<Request<()>, Status> {
        match req.metadata().get("authorization") {
            Some(t) if &self.token == t => Ok(req),
            _ => Err(Status::unauthenticated("No valid auth token")),
        }
    }
}
