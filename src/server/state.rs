use crate::config::Settings;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub config: Settings,
}

impl AppState {
    pub fn new(config: Settings) -> Arc<Self> {
        Arc::new(Self { config })
    }
}
