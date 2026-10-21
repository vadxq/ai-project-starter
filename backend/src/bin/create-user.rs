//! 显式创建账号，不覆盖已有用户名，也不在 API 启动时写入数据。
use anyhow::Context;
use rust_axum_starter::platform::{auth::create_account, config::Settings, db::DatabasePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let username = std::env::args()
        .nth(1)
        .context("usage: create-user <username>")?;
    let password =
        std::env::var("STARTER_USER_PASSWORD").context("STARTER_USER_PASSWORD is required")?;
    let settings = Settings::load()?;
    let database = DatabasePool::from_settings(&settings.database)?
        .context("APP_DATABASE__URL is required")?;
    let result = create_account(&database, username, password).await;
    database.close().await;
    result?;
    Ok(())
}
