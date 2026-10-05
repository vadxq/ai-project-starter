use std::{env, net::SocketAddr, time::Duration};

use axum::http::HeaderValue;
use url::Url;

pub const IO_TIMEOUT: Duration = Duration::from_secs(5);
pub const CONNECT_ATTEMPTS: usize = 3;

pub struct Config {
    pub database_url: String,
    pub bind_address: SocketAddr,
    pub jwt_secret: String,
    pub cors_origins: Vec<HeaderValue>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing or invalid configuration: {0}")]
    Invalid(&'static str),
}

pub fn required(name: &'static str) -> Result<String, ConfigError> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or(ConfigError::Invalid(name))
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let jwt_secret = required("JWT_SECRET")?;
        if jwt_secret.len() < 32 || jwt_secret.starts_with("replace-") {
            return Err(ConfigError::Invalid(
                "JWT_SECRET requires a random secret of at least 32 bytes",
            ));
        }
        let cors_origins = required("CORS_ORIGINS")?
            .split(',')
            .map(|origin| {
                let parsed =
                    Url::parse(origin).map_err(|_| ConfigError::Invalid("CORS_ORIGINS"))?;
                if !matches!(parsed.scheme(), "http" | "https")
                    || parsed.path() != "/"
                    || parsed.query().is_some()
                    || parsed.fragment().is_some()
                {
                    return Err(ConfigError::Invalid("CORS_ORIGINS"));
                }
                origin
                    .parse()
                    .map_err(|_| ConfigError::Invalid("CORS_ORIGINS"))
            })
            .collect::<Result<Vec<HeaderValue>, ConfigError>>()?;
        Ok(Self {
            database_url: required("DATABASE_URL")?,
            bind_address: required("BIND_ADDRESS")?
                .parse()
                .map_err(|_| ConfigError::Invalid("BIND_ADDRESS"))?,
            jwt_secret,
            cors_origins,
        })
    }
}
