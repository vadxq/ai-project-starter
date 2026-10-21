use utoipa::{
    Modify, OpenApi,
    openapi::{
        Components, OpenApi as Specification, Ref, RefOr, Schema,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

use super::error::ErrorResponse;
use crate::domain::auth::{LoginRequest, RefreshRequest, TokenResponse};
use crate::domain::models::*;
use crate::domain::{auth::handler as auth, health::handler as health, items::handler as routes};

#[derive(OpenApi)]
#[openapi(
    info(title = "Starter API", version = "0.1.0", description = "Username/password login issues a 15-minute JWT and a rotating refresh token. Logout revokes the session. Titles use 1–240 UTF-8 bytes after ASCII trimming. PATCH rejects null and requires at least one changed field. Responses use code/message/data; errors use code/message/details. Timestamps are epoch milliseconds. PostgreSQL and MySQL share business semantics."),
    paths(auth::login, auth::refresh, auth::logout, health::health, health::ready, routes::me, routes::list, routes::create, routes::get, routes::update, routes::delete),
    components(schemas(LoginRequest, RefreshRequest, TokenResponse, Identity, Item, ItemPage, CreateItem, UpdateItem, ErrorResponse, health::HealthResponse, health::ReadyResponse)),
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
            for name in [
                "Identity",
                "Item",
                "ItemPage",
                "TokenResponse",
                "HealthResponse",
                "ReadyResponse",
            ] {
                link_response(components, name);
            }
        }
    }
}

/// 泛型包装复用命名 DTO，避免各生成器为内联对象生成另一套不兼容类型。
fn link_response(components: &mut Components, name: &str) {
    let wrapper = format!("SuccessResponse_{name}");
    if let Some(RefOr::T(Schema::Object(schema))) = components.schemas.get_mut(&wrapper) {
        schema
            .properties
            .insert("data".to_owned(), Ref::from_schema_name(name).into());
    }
}
