use super::{LoginRequest, RefreshRequest, TokenResponse};
use crate::{
    api::{
        context::RequestContext,
        error::{AppError, ErrorResponse},
        response::{self, ApiJson, AppJson, SuccessResponse},
    },
    bootstrap::AppState,
};
use axum::{Extension, extract::State, http::StatusCode};

#[utoipa::path(post, path = "/api/v1/auth/login", tag = "auth", operation_id = "login", request_body = LoginRequest,
    responses((status = 200, body = SuccessResponse<TokenResponse>), (status = 400, body = ErrorResponse), (status = 401, body = ErrorResponse), (status = 413, body = ErrorResponse), (status = 415, body = ErrorResponse), (status = 422, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn login(
    State(state): State<AppState>,
    Extension(context): Extension<RequestContext>,
    AppJson(input): AppJson<LoginRequest>,
) -> Result<ApiJson<TokenResponse>, AppError> {
    context
        .tokens(&state)?
        .login(context.database(&state)?, input)
        .await
        .map(response::ok)
        .map_err(|error| context.auth_error(error))
}

#[utoipa::path(post, path = "/api/v1/auth/refresh", tag = "auth", operation_id = "refreshSession", request_body = RefreshRequest,
    responses((status = 200, body = SuccessResponse<TokenResponse>), (status = 400, body = ErrorResponse), (status = 401, body = ErrorResponse), (status = 413, body = ErrorResponse), (status = 415, body = ErrorResponse), (status = 422, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn refresh(
    State(state): State<AppState>,
    Extension(context): Extension<RequestContext>,
    AppJson(input): AppJson<RefreshRequest>,
) -> Result<ApiJson<TokenResponse>, AppError> {
    context
        .tokens(&state)?
        .refresh(context.database(&state)?, &input.refresh_token)
        .await
        .map(response::ok)
        .map_err(|error| context.auth_error(error))
}

#[utoipa::path(post, path = "/api/v1/auth/logout", tag = "auth", operation_id = "logout", request_body = RefreshRequest,
    responses((status = 204), (status = 400, body = ErrorResponse), (status = 401, body = ErrorResponse), (status = 413, body = ErrorResponse), (status = 415, body = ErrorResponse), (status = 422, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn logout(
    State(state): State<AppState>,
    Extension(context): Extension<RequestContext>,
    AppJson(input): AppJson<RefreshRequest>,
) -> Result<StatusCode, AppError> {
    context
        .tokens(&state)?
        .logout(context.database(&state)?, &input.refresh_token)
        .await
        .map_err(|error| context.auth_error(error))?;
    Ok(StatusCode::NO_CONTENT)
}
