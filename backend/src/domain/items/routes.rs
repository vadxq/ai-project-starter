use super::handler;
use crate::bootstrap::AppState;
use axum::{
    Router,
    routing::{get, patch},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/me", get(handler::me))
        .route("/api/v1/items", get(handler::list).post(handler::create))
        .route(
            "/api/v1/items/{id}",
            patch(handler::update)
                .get(handler::get)
                .delete(handler::delete),
        )
}
