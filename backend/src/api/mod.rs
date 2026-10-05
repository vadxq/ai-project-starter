pub mod auth;
pub mod context;
pub mod error;
pub mod openapi;
pub mod routes;

use axum::{
    Extension, Router,
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method, header},
    middleware,
    routing::{get, patch, post},
};
use tower_http::cors::CorsLayer;

use crate::AppState;
use context::RequestContext;
use error::{ApiError, ErrorKind};

const MAX_BODY_BYTES: usize = 16 * 1024;

async fn not_found(Extension(context): Extension<RequestContext>) -> ApiError {
    context.error(ErrorKind::NotFound)
}

async fn method_not_allowed(Extension(context): Extension<RequestContext>) -> ApiError {
    context.error(ErrorKind::MethodNotAllowed)
}

pub fn router(state: AppState, origins: Vec<HeaderValue>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT_LANGUAGE,
        ])
        .expose_headers([
            header::LOCATION,
            header::HeaderName::from_static("x-request-id"),
        ]);
    Router::new()
        .route("/health", get(routes::health))
        .route("/ready", get(routes::ready))
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/refresh", post(auth::refresh))
        .route("/api/v1/auth/logout", post(auth::logout))
        .route("/api/v1/me", get(routes::me))
        .route("/api/v1/items", get(routes::list).post(routes::create))
        .route(
            "/api/v1/items/{id}",
            patch(routes::update)
                .get(routes::get)
                .delete(routes::delete),
        )
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .layer(cors)
        .layer(middleware::from_fn(context::request_context))
        .with_state(state)
}
