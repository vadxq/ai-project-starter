//! 业务语言与鉴权；请求 ID / trace / CORS 由上游中间件负责。
use super::error::AppError;
use crate::{
    bootstrap::AppState,
    domain::{items::StoreError, models::Identity},
    platform::{
        auth::{AuthError, TokenService},
        db::DatabasePool,
    },
};
use axum::{
    extract::{FromRequestParts, Request},
    http::{StatusCode, header, request::Parts},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestContext {
    pub locale: &'static str,
}

pub fn locale(value: &str) -> &'static str {
    let mut languages: Vec<(&str, f32)> = value
        .split(',')
        .filter_map(|part| {
            let mut pieces = part.trim().split(';');
            let language = pieces.next()?.trim();
            let quality = pieces
                .next()
                .map_or(Some(1.0), |q| q.trim().strip_prefix("q=")?.parse().ok())?;
            (quality > 0.0 && quality <= 1.0).then_some((language, quality))
        })
        .collect();
    languages.sort_by(|left, right| right.1.total_cmp(&left.1));
    for (language, _) in languages {
        match language.to_ascii_lowercase().as_str() {
            "zh" | "zh-cn" | "zh-hans" => return "zh-CN",
            "en" | "en-us" | "en-gb" | "*" => return "en",
            _ => continue,
        }
    }
    "en"
}

pub async fn context(mut request: Request, next: Next) -> Response {
    let context = RequestContext {
        locale: locale(
            request
                .headers()
                .get(header::ACCEPT_LANGUAGE)
                .and_then(|value| value.to_str().ok())
                .unwrap_or("en"),
        ),
    };
    request.extensions_mut().insert(context);
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store"),
    );
    response
}

impl RequestContext {
    pub fn error(&self, status: StatusCode, reason: &'static str) -> AppError {
        AppError::new(
            status,
            rust_i18n::t!(reason, locale = self.locale).into_owned(),
        )
        .with_details(serde_json::json!({"reason": reason}))
    }

    pub fn database<'a>(&self, state: &'a AppState) -> Result<&'a DatabasePool, AppError> {
        state
            .health
            .database
            .as_ref()
            .ok_or_else(|| self.error(StatusCode::SERVICE_UNAVAILABLE, "service_unavailable"))
    }

    pub fn tokens<'a>(&self, state: &'a AppState) -> Result<&'a Arc<TokenService>, AppError> {
        state
            .tokens
            .as_ref()
            .ok_or_else(|| self.error(StatusCode::SERVICE_UNAVAILABLE, "service_unavailable"))
    }

    pub fn store_error(&self, error: StoreError) -> AppError {
        match error {
            StoreError::NotFound => self.error(StatusCode::NOT_FOUND, "not_found"),
            StoreError::Conflict => self.error(StatusCode::CONFLICT, "version_conflict"),
            StoreError::Database(error) => self.database_error(error),
        }
    }

    fn database_error(&self, error: sqlx::Error) -> AppError {
        // 不记录连接字符串和用户输入；连接故障与内部 SQL 错误分别处理。
        tracing::error!(event = "business_database_error", category = ?error.as_database_error().and_then(|value| value.code()));
        match error {
            sqlx::Error::PoolTimedOut
            | sqlx::Error::PoolClosed
            | sqlx::Error::Io(_)
            | sqlx::Error::Tls(_) => {
                self.error(StatusCode::SERVICE_UNAVAILABLE, "service_unavailable")
            }
            _ => self.error(StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        }
    }

    pub fn auth_error(&self, error: AuthError) -> AppError {
        match error {
            AuthError::Invalid => self.error(StatusCode::UNAUTHORIZED, "unauthorized"),
            AuthError::Credentials => self.error(StatusCode::UNAUTHORIZED, "invalid_credentials"),
            AuthError::Validation => self.error(StatusCode::BAD_REQUEST, "validation_error"),
            AuthError::Database(error) => self.database_error(error),
            AuthError::Internal => self.error(StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        }
    }
}

pub struct Authenticated {
    pub identity: Identity,
    pub context: RequestContext,
    pub database: DatabasePool,
}

impl FromRequestParts<AppState> for Authenticated {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let context = parts
            .extensions
            .get::<RequestContext>()
            .expect("business context middleware")
            .clone();
        let database = context.database(state)?;
        let tokens = context.tokens(state)?;
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .filter(|value| !value.is_empty())
            .ok_or_else(|| context.error(StatusCode::UNAUTHORIZED, "unauthorized"))?;
        let identity = tokens
            .verify(database, token)
            .await
            .map_err(|error| context.auth_error(error))?;
        Ok(Self {
            identity,
            context,
            database: database.clone(),
        })
    }
}
