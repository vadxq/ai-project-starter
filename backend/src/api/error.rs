//! API 对外错误模型与通用转换。

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;

/// handler 与 service 统一使用的结果类型。
pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone)]
/// 业务错误对象，会被统一序列化为 JSON 响应。
pub struct AppError {
    pub status: StatusCode,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = ErrorDetails, nullable = false)]
    pub details: Option<serde_json::Value>,
}

/// 上游 details 的已知字段，仅补充契约元数据，运行时仍保留可扩展 JSON 对象。
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorDetails {
    pub reason: Option<String>,
    pub rejection: Option<String>,
    pub source_status: Option<u16>,
    pub error: Option<String>,
}

impl AppError {
    /// 创建 400 错误。
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    /// 创建 401 错误。
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, message)
    }

    pub fn method_not_allowed(message: impl Into<String>) -> Self {
        Self::new(StatusCode::METHOD_NOT_ALLOWED, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, message)
    }

    /// 附加结构化错误详情，便于前端或测试定位原因。
    pub fn with_details(self, details: serde_json::Value) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            details: None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // 对外只暴露统一错误结构，避免 handler 各自拼装响应格式。
        let body = ErrorResponse {
            code: self.status.as_u16().to_string(),
            message: self.message,
            details: self.details,
        };
        let mut response = (self.status, Json(body)).into_response();
        if self.status == StatusCode::UNAUTHORIZED {
            response.headers_mut().insert(
                axum::http::header::WWW_AUTHENTICATE,
                axum::http::HeaderValue::from_static("Bearer"),
            );
        }
        response
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!(error = %value, "database error");
        Self::internal("database error")
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        tracing::error!(error = %value, "internal error");
        Self::internal("internal server error")
    }
}
