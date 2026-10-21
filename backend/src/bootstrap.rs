//! 将配置转换为运行时 `AppState`。

use std::sync::Arc;

use crate::{
    domain::health::HealthDomain,
    platform::{auth::TokenService, config::Settings, db::DatabasePool},
};

#[derive(Clone)]
/// 提供给路由和 handler 共享的 domain 聚合对象。
pub struct AppState {
    pub health: HealthDomain,
    pub tokens: Option<Arc<TokenService>>,
}

/// 根据配置创建可选数据库连接，并组装各领域依赖。
pub fn build_state(settings: &Settings) -> Result<AppState, sqlx::Error> {
    let database: Option<DatabasePool> = DatabasePool::from_settings(&settings.database)?;
    Ok(AppState {
        tokens: settings
            .auth
            .jwt_secret
            .as_deref()
            .map(TokenService::new)
            .transpose()
            .map_err(|_| sqlx::Error::Configuration("authentication initialization failed".into()))?
            .map(Arc::new),
        health: HealthDomain::new(
            database,
            std::time::Duration::from_secs(settings.database.timeout_secs.get()),
        ),
    })
}
