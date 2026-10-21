//! 组装应用的根路由与通用 HTTP 中间件。

use axum::{Router, middleware};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};

use crate::{api::error::AppError, bootstrap::AppState, domain};

/// 创建整个 API 的根路由树，并挂载全局中间件。
pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(domain::health::routes::router())
        .merge(domain::auth::routes::router())
        .merge(domain::items::routes::router())
        .fallback(|| async { Err::<(), _>(AppError::not_found("route not found")) })
        .method_not_allowed_fallback(|| async {
            Err::<(), _>(AppError::method_not_allowed("method not allowed"))
        })
        .with_state(state)
        .layer(middleware::from_fn(super::context::context))
        // ServiceBuilder 按声明顺序接收请求；ID 必须先于 trace 和响应传播。
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &axum::http::Request<axum::body::Body>| {
                            tracing::info_span!(
                                "http.request",
                                method = %request.method(),
                                path = request.uri().path(),
                                request_id = request.headers().get("x-request-id")
                                    .and_then(|value| value.to_str().ok()).unwrap_or(""),
                            )
                        })
                        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
                )
                .layer(PropagateRequestIdLayer::x_request_id())
                .layer(CorsLayer::permissive()),
        )
}
