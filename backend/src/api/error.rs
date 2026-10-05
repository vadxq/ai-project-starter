use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{domain::items::StoreError, platform::auth::AuthError};

use super::context::RequestContext;

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Validation,
    Unauthorized,
    InvalidCredentials,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    Conflict,
    TooLarge,
    Unavailable,
    Internal,
}

impl ErrorKind {
    pub fn status(self) -> StatusCode {
        match self {
            Self::Validation => StatusCode::BAD_REQUEST,
            Self::Unauthorized | Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Self::Conflict => StatusCode::CONFLICT,
            Self::TooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Validation => "validation_error",
            Self::Unauthorized => "unauthorized",
            Self::InvalidCredentials => "invalid_credentials",
            Self::Forbidden => "forbidden",
            Self::NotFound => "not_found",
            Self::MethodNotAllowed => "method_not_allowed",
            Self::Conflict => "version_conflict",
            Self::TooLarge => "payload_too_large",
            Self::Unavailable => "service_unavailable",
            Self::Internal => "internal_error",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    pub code: String,
    pub detail: String,
    pub instance: String,
    pub request_id: String,
}

#[derive(Debug)]
pub struct ApiError {
    pub context: RequestContext,
    pub kind: ErrorKind,
}

impl RequestContext {
    pub fn error(&self, kind: ErrorKind) -> ApiError {
        ApiError {
            context: self.clone(),
            kind,
        }
    }

    pub fn store_error(&self, error: StoreError) -> ApiError {
        let kind = match error {
            StoreError::NotFound => ErrorKind::NotFound,
            StoreError::Conflict => ErrorKind::Conflict,
            StoreError::Database(error) => {
                // 不记录数据库 URL 或参数；requestId 将服务器故障和客户端报告关联。
                tracing::error!(event = "database_operation_failed", request_id = %self.id, category = ?error.as_database_error().map(|e| e.code()));
                ErrorKind::Unavailable
            }
        };
        self.error(kind)
    }

    pub fn auth_error(&self, error: AuthError) -> ApiError {
        self.error(match error {
            AuthError::Invalid => ErrorKind::Unauthorized,
            AuthError::Credentials => ErrorKind::InvalidCredentials,
            AuthError::Validation => ErrorKind::Validation,
            AuthError::Database(_) => ErrorKind::Unavailable,
            AuthError::Internal => ErrorKind::Internal,
        })
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.kind.status();
        let code = self.kind.code();
        let title = rust_i18n::t!(code, locale = self.context.locale).into_owned();
        let problem = Problem {
            r#type: format!("urn:ai-project-starter:problem:{code}"),
            title: title.clone(),
            detail: title,
            status: status.as_u16(),
            code: code.to_owned(),
            instance: self.context.path,
            request_id: self.context.id.to_string(),
        };
        let mut response = (status, Json(problem)).into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/problem+json"),
        );
        if status == StatusCode::UNAUTHORIZED {
            response.headers_mut().insert(
                header::WWW_AUTHENTICATE,
                header::HeaderValue::from_static("Bearer"),
            );
        }
        response
    }
}
