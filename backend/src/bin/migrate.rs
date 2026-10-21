//! 显式执行当前数据库方言的 migration，并记录 SQLx 版本与校验和。

use anyhow::Context;
use rust_axum_starter::platform::{config::Settings, db::DatabasePool, telemetry::init_tracing};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings: Settings = Settings::load()?;
    init_tracing(&settings)?;
    let database: DatabasePool = DatabasePool::from_settings(&settings.database)?
        .context("执行迁移前请设置 APP_DATABASE__URL")?;
    let result = database.migrate().await;
    database.close().await;
    result?;
    tracing::info!(database = database.kind(), "database migrations complete");
    Ok(())
}
