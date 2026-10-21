//! 健康检查路由定义。

use axum::{Router, routing::get};

use crate::bootstrap::AppState;

use super::handler;

/// 注册健康检查端点。
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(handler::health))
        .route("/ready", get(handler::ready))
}
