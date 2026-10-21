//! 按 URL 选择强类型连接池；两种 SQL 方言与迁移历史分别维护。

use std::{path::Path, time::Duration};

use sqlx::{
    MySqlPool, PgPool,
    migrate::{MigrateError, Migrator},
    mysql::MySqlPoolOptions,
    postgres::PgPoolOptions,
};

use crate::platform::config::DatabaseSettings;

pub mod sql;

#[derive(Clone)]
pub enum DatabasePool {
    Postgres(PgPool),
    MySql(MySqlPool),
}

impl DatabasePool {
    /// lazy pool 不要求数据库在启动时可用，后续请求会重新尝试连接。
    pub fn from_settings(settings: &DatabaseSettings) -> Result<Option<Self>, sqlx::Error> {
        let Some(url) = settings.url.as_deref() else {
            return Ok(None);
        };
        let timeout: Duration = Duration::from_secs(settings.timeout_secs.get());
        let pool: Self = match url.split_once("://").map(|(scheme, _)| scheme) {
            Some("postgres" | "postgresql") => Self::Postgres(
                PgPoolOptions::new()
                    .max_connections(settings.max_connections.get())
                    .acquire_timeout(timeout)
                    .connect_lazy(url)
                    .map_err(|_| invalid_database_url())?,
            ),
            Some("mysql") => Self::MySql(
                MySqlPoolOptions::new()
                    .max_connections(settings.max_connections.get())
                    .acquire_timeout(timeout)
                    .connect_lazy(url)
                    .map_err(|_| invalid_database_url())?,
            ),
            _ => return Err(invalid_database_url()),
        };
        Ok(Some(pool))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Postgres(_) => "postgres",
            Self::MySql(_) => "mysql",
        }
    }

    /// 探针语义相同，业务查询由领域 repo 分别编写两种 SQL。
    pub async fn ping(&self) -> Result<(), sqlx::Error> {
        match self {
            Self::Postgres(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
            Self::MySql(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
        }
        Ok(())
    }

    pub fn migrations_path(&self) -> &'static Path {
        match self {
            Self::Postgres(_) => Path::new("migrations/postgres"),
            Self::MySql(_) => Path::new("migrations/mysql"),
        }
    }

    /// 仅由显式迁移命令调用，HTTP 启动不自动执行 DDL。
    pub async fn migrate(&self) -> Result<(), MigrateError> {
        let migrator: Migrator = Migrator::new(self.migrations_path()).await?;
        match self {
            Self::Postgres(pool) => migrator.run(pool).await,
            Self::MySql(pool) => migrator.run(pool).await,
        }
    }

    pub async fn close(&self) {
        match self {
            Self::Postgres(pool) => pool.close().await,
            Self::MySql(pool) => pool.close().await,
        }
    }
}

/// 不把 URL 或密码纳入错误上下文。
fn invalid_database_url() -> sqlx::Error {
    sqlx::Error::Configuration(
        "APP_DATABASE__URL 必须是有效的 postgres://、postgresql:// 或 mysql:// 地址".into(),
    )
}

/// 使用驱动识别 SQLSTATE，避免硬编码单一数据库错误码。
pub fn is_unique_violation(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .is_some_and(|error| error.is_unique_violation())
}
