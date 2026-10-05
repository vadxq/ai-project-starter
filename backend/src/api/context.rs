use std::time::Instant;

use axum::{
    extract::{FromRequestParts, Request},
    http::{HeaderValue, header, request::Parts},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{AppState, domain::models::Identity};

use super::error::{ApiError, ErrorKind};

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub id: Uuid,
    pub path: String,
    pub locale: &'static str,
}

pub fn locale(header: &str) -> &'static str {
    let mut languages: Vec<(&str, f32)> = header
        .split(',')
        .filter_map(|part| {
            let mut pieces = part.trim().split(';');
            let language = pieces.next()?.trim();
            let quality = pieces.next().map_or(Some(1.0), |q| {
                q.trim().strip_prefix("q=")?.parse::<f32>().ok()
            })?;
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

pub async fn request_context(mut request: Request, next: Next) -> Response {
    let context = RequestContext {
        id: Uuid::new_v4(),
        path: request.uri().path().to_owned(),
        locale: locale(
            request
                .headers()
                .get(header::ACCEPT_LANGUAGE)
                .and_then(|h| h.to_str().ok())
                .unwrap_or("en"),
        ),
    };
    let started = Instant::now();
    let method = request.method().clone();
    request.extensions_mut().insert(context.clone());
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&context.id.to_string()).expect("UUID header"),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    tracing::info!(event = "http_request", request_id = %context.id, method = %method, path = %context.path,
        status = response.status().as_u16(), latency_ms = started.elapsed().as_millis() as u64);
    response
}

pub struct Authenticated {
    pub identity: Identity,
    pub context: RequestContext,
    pub state: AppState,
}

impl FromRequestParts<AppState> for Authenticated {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let context = parts
            .extensions
            .get::<RequestContext>()
            .expect("request context middleware")
            .clone();
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .filter(|value| !value.is_empty())
            .ok_or_else(|| context.error(ErrorKind::Unauthorized))?;
        let identity = state
            .tokens
            .verify(&state.pool, token)
            .await
            .map_err(|error| context.auth_error(error))?;
        Ok(Self {
            identity,
            context,
            state: state.clone(),
        })
    }
}
