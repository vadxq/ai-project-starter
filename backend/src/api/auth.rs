use axum::{
    Extension, Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
};

use super::{
    context::RequestContext,
    error::{ApiError, ErrorKind, Problem},
};
use crate::{
    AppState,
    domain::auth::{LoginRequest, RefreshRequest, TokenResponse},
};

#[utoipa::path(post, path = "/api/v1/auth/login", operation_id = "login", request_body = LoginRequest,
    responses((status = 200, body = TokenResponse), (status = 400, body = Problem), (status = 401, body = Problem), (status = 503, body = Problem)))]
pub async fn login(
    State(state): State<AppState>,
    Extension(context): Extension<RequestContext>,
    input: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Json<TokenResponse>, ApiError> {
    let Json(input) = input.map_err(|_| context.error(ErrorKind::Validation))?;
    state
        .tokens
        .login(&state.pool, input)
        .await
        .map(Json)
        .map_err(|error| context.auth_error(error))
}

#[utoipa::path(post, path = "/api/v1/auth/refresh", operation_id = "refreshSession", request_body = RefreshRequest,
    responses((status = 200, body = TokenResponse), (status = 400, body = Problem), (status = 401, body = Problem), (status = 503, body = Problem)))]
pub async fn refresh(
    State(state): State<AppState>,
    Extension(context): Extension<RequestContext>,
    input: Result<Json<RefreshRequest>, JsonRejection>,
) -> Result<Json<TokenResponse>, ApiError> {
    let Json(input) = input.map_err(|_| context.error(ErrorKind::Validation))?;
    state
        .tokens
        .refresh(&state.pool, &input.refresh_token)
        .await
        .map(Json)
        .map_err(|error| context.auth_error(error))
}

#[utoipa::path(post, path = "/api/v1/auth/logout", operation_id = "logout", request_body = RefreshRequest,
    responses((status = 204), (status = 400, body = Problem), (status = 401, body = Problem), (status = 503, body = Problem)))]
pub async fn logout(
    State(state): State<AppState>,
    Extension(context): Extension<RequestContext>,
    input: Result<Json<RefreshRequest>, JsonRejection>,
) -> Result<StatusCode, ApiError> {
    let Json(input) = input.map_err(|_| context.error(ErrorKind::Validation))?;
    state
        .tokens
        .logout(&state.pool, &input.refresh_token)
        .await
        .map_err(|error| context.auth_error(error))?;
    Ok(StatusCode::NO_CONTENT)
}
