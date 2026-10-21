//! API 统一成功响应、时间序列化与 extractor 适配。

use std::ops::{Deref, DerefMut};

use axum::{
    extract::{FromRequest, Json, Request, rejection::JsonRejection},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Serialize, de::DeserializeOwned};
use utoipa::ToSchema;

use crate::api::error::AppError;

#[derive(Debug, Serialize, ToSchema)]
/// 对外统一的成功响应结构。
pub struct SuccessResponse<T> {
    /// HTTP 状态码字符串，例如 `"200"`。
    pub code: String,
    /// 成功消息，当前统一固定为 `success`。
    pub message: &'static str,
    /// 真实业务数据载荷。
    pub data: T,
}

impl<T> SuccessResponse<T> {
    /// 构造标准 200 成功响应。
    pub fn ok(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16().to_string(),
            message: "success",
            data,
        }
    }

    /// 创建资源时响应包装中的 code 必须与真实 HTTP 状态一致。
    pub fn created(data: T) -> Self {
        Self {
            code: "201".to_owned(),
            message: "success",
            data,
        }
    }
}

/// 统一的成功 JSON 响应类型别名。
pub type ApiJson<T> = Json<SuccessResponse<T>>;

/// 将业务 DTO 包装成统一成功响应。
pub fn ok<T>(data: T) -> ApiJson<T> {
    Json(SuccessResponse::ok(data))
}

#[derive(Debug)]
/// 包装 `axum::Json<T>`，把 JSON rejection 转成统一错误结构。
pub struct AppJson<T>(pub T);

impl<T> Deref for AppJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for AppJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<S, T> FromRequest<S> for AppJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(Self(value)),
            Err(err) => Err(map_json_rejection(err)),
        }
    }
}

/// 把时间序列化为 13 位毫秒时间戳。
pub fn serialize_timestamp_millis<S>(
    value: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_i64(value.timestamp_millis())
}

/// 把可选时间序列化为 13 位毫秒时间戳；为空时输出 `null`。
pub fn serialize_option_timestamp_millis<S>(
    value: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(value) => serializer.serialize_some(&value.timestamp_millis()),
        None => serializer.serialize_none(),
    }
}

fn map_json_rejection(err: JsonRejection) -> AppError {
    let details = serde_json::json!({
        "rejection": json_rejection_kind(&err),
        "source_status": err.status().as_u16(),
        "error": err.body_text(),
    });

    AppError::new(err.status(), "invalid json body").with_details(details)
}

fn json_rejection_kind(err: &JsonRejection) -> &'static str {
    match err {
        JsonRejection::JsonDataError(_) => "json_data_error",
        JsonRejection::JsonSyntaxError(_) => "json_syntax_error",
        JsonRejection::MissingJsonContentType(_) => "missing_json_content_type",
        JsonRejection::BytesRejection(_) => "bytes_rejection",
        _ => "unknown_json_rejection",
    }
}
