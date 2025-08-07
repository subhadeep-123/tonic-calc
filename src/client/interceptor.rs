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
    pub fn new(token: &str) -> Result<Self, Status> {
        Ok(Self {
            token: format!("Bearer {}", token)
                .parse()
                .map_err(|_| Status::internal("Invalid Token"))?,
        })
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        req.metadata_mut()
            .insert("authorization", self.token.clone());
        Ok(req)
    }
}
