use utoipa::{
    Modify, OpenApi,
    openapi::{
        OpenApi as Specification,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

use super::{auth, error::Problem, routes};
use crate::domain::auth::{LoginRequest, RefreshRequest, TokenResponse};
use crate::domain::models::*;

#[derive(OpenApi)]
#[openapi(
    info(title = "Starter API", version = "0.1.0", description = "Username/password login issues a 15-minute JWT and a rotating refresh token. Logout revokes the session. Titles use 1–240 UTF-8 bytes after ASCII trimming. PATCH rejects null and requires at least one changed field. All errors use RFC9457 Problem Details."),
    paths(auth::login, auth::refresh, auth::logout, routes::health, routes::ready, routes::me, routes::list, routes::create, routes::get, routes::update, routes::delete),
    components(schemas(LoginRequest, RefreshRequest, TokenResponse, Identity, Item, ItemPage, CreateItem, UpdateItem, Problem, Health)),
    modifiers(&ContractMetadata)
)]
pub struct ApiDoc;

struct ContractMetadata;

impl Modify for ContractMetadata {
    fn modify(&self, openapi: &mut Specification) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
        // 使用同一错误 media type，并显式记录路由 / method / 内部错误约定。
        for path in openapi.paths.paths.values_mut() {
            let operations = [
                &mut path.get,
                &mut path.post,
                &mut path.patch,
                &mut path.delete,
            ];
            for operation in operations.into_iter().flatten() {
                for (status, response) in &mut operation.responses.responses {
                    if status.parse::<u16>().is_ok_and(|code| code >= 400)
                        && let utoipa::openapi::RefOr::T(response) = response
                        && let Some(content) = response.content.shift_remove("application/json")
                    {
                        response
                            .content
                            .insert("application/problem+json".to_owned(), content);
                    }
                }
            }
        }
    }
}
