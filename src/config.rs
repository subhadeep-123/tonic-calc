use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub client: ClientConfig,
    pub auth: AuthConfig,
    pub observability: ObservabilityConfig,
    pub tls: TlsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub max_requests_per_sec: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientConfig {
    pub address: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    #[serde(default)]
    pub enable_auth: bool,

    #[serde(default)]
    pub auth_token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TlsConfig {
    pub path: String,
    pub domain_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ObservabilityConfig {
    #[serde(default)]
    pub enable_tracing: bool,
}

pub fn load() -> Result<Settings, config::ConfigError> {
    // Load from .env file first
    let _ = dotenvy::dotenv();

    // Load from Config.toml and environment variables with "APP__" prefix
    let builder = config::Config::builder()
        .add_source(config::File::with_name("Config").required(false)) // load Config.toml
        .add_source(config::Environment::with_prefix("APP").separator("__")); // load env variables

    let config = builder.build()?;

    config.try_deserialize()
}
