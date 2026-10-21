use super::handler;
use crate::bootstrap::AppState;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/auth/login", post(handler::login))
        .route("/api/v1/auth/refresh", post(handler::refresh))
        .route("/api/v1/auth/logout", post(handler::logout))
}
