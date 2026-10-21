//! 健康检查领域，负责暴露存活与就绪探针。

pub mod handler;
pub mod routes;

use std::time::Duration;

use crate::platform::db::DatabasePool;

#[derive(Clone)]
/// 健康检查依赖；数据库连接为空时表示当前服务只启动 HTTP 层。
pub struct HealthDomain {
    pub database: Option<DatabasePool>,
    pub timeout: Duration,
}

impl HealthDomain {
    /// 构造健康检查领域实例。
    pub fn new(database: Option<DatabasePool>, timeout: Duration) -> Self {
        Self { database, timeout }
    }
}
