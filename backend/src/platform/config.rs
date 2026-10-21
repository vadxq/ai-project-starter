//! 以只读快照合并配置，避免修改进程环境造成并发风险。

use std::{
    collections::HashMap,
    net::SocketAddr,
    num::{NonZeroU32, NonZeroU64},
};

use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use tracing_subscriber::EnvFilter;

/// 数据库凭证不派生 Debug，避免意外进入日志。
#[derive(Clone, Deserialize)]
pub struct Settings {
    pub http: HttpSettings,
    pub database: DatabaseSettings,
    pub telemetry: TelemetrySettings,
    pub auth: AuthSettings,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthSettings {
    /// 不配置密钥时可运行探针，业务认证返回依赖不可用。
    pub jwt_secret: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpSettings {
    pub bind_addr: SocketAddr,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseSettings {
    pub url: Option<String>,
    pub max_connections: NonZeroU32,
    /// 同时约束连接获取与就绪查询的总时限。
    pub timeout_secs: NonZeroU64,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetrySettings {
    pub log_level: String,
    pub json: bool,
}

impl Settings {
    /// 优先级：默认值 < 当前目录 .env < 进程环境；损坏的 .env 必须报错。
    pub fn load() -> Result<Self, ConfigError> {
        let dotenv: HashMap<String, String> = match dotenvy::from_path_iter(".env") {
            Ok(iter) => iter
                .collect::<Result<HashMap<String, String>, dotenvy::Error>>()
                .map_err(|_| ConfigError::Message(".env 格式错误，请检查配置文件".into()))?,
            Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                HashMap::new()
            }
            Err(_) => return Err(ConfigError::Message("无法读取 .env 配置文件".into())),
        };
        Self::from_sources(dotenv, std::env::vars().collect())
    }

    /// 显式输入验证配置优先级，不读写全局环境。
    pub fn from_sources(
        dotenv: HashMap<String, String>,
        environment: HashMap<String, String>,
    ) -> Result<Self, ConfigError> {
        let source: HashMap<String, String> = dotenv
            .into_iter()
            .chain(environment)
            .map(|(key, value)| (key, value.trim().to_owned()))
            .collect();
        let log_level: String = source
            .get("RUST_LOG")
            .filter(|value| !value.is_empty())
            .or_else(|| source.get("APP_TELEMETRY__LOG_LEVEL"))
            .filter(|value| !value.is_empty())
            .cloned()
            .unwrap_or_else(|| "info".to_owned());
        EnvFilter::try_new(&log_level)
            .map_err(|_| ConfigError::Message("日志过滤器格式错误".into()))?;

        let settings: Self = Config::builder()
            .set_default("http.bind_addr", "0.0.0.0:8080")?
            .set_default("database.url", None::<String>)?
            .set_default("database.max_connections", 10)?
            .set_default("database.timeout_secs", 3)?
            .set_default("telemetry.log_level", "info")?
            .set_default("telemetry.json", false)?
            .set_default("auth.jwt_secret", None::<String>)?
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true)
                    .ignore_empty(true)
                    .source(Some(source)),
            )
            .set_override("telemetry.log_level", log_level)?
            .build()?
            .try_deserialize()?;
        if settings
            .auth
            .jwt_secret
            .as_ref()
            .is_some_and(|secret| secret.len() < 32 || secret.starts_with("replace-"))
        {
            return Err(ConfigError::Message(
                "APP_AUTH__JWT_SECRET 必须为至少 32 字节的随机密钥".into(),
            ));
        }
        Ok(settings)
    }
}
