use axum::{
    Extension, Json,
    extract::{
        Path, Query, State,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
    AppState,
    domain::{
        items::{self, UpdateCommand},
        models::*,
    },
};

use super::{
    context::{Authenticated, RequestContext},
    error::{ApiError, ErrorKind, Problem},
};

fn json_error(context: &RequestContext, rejection: JsonRejection) -> ApiError {
    context.error(if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE {
        ErrorKind::TooLarge
    } else {
        ErrorKind::Validation
    })
}

#[utoipa::path(get, path = "/health", operation_id = "health", responses((status = 200, body = Health)))]
pub async fn health() -> Json<Health> {
    Json(Health {
        status: "ok".to_owned(),
    })
}

#[utoipa::path(get, path = "/ready", operation_id = "ready", responses((status = 200, body = Health), (status = 503, body = Problem, content_type = "application/problem+json")))]
pub async fn ready(
    State(state): State<AppState>,
    Extension(context): Extension<RequestContext>,
) -> Result<Json<Health>, ApiError> {
    sqlx::query("SELECT 1")
        .execute(&state.pool)
        .await
        .map_err(|error| context.store_error(error.into()))?;
    Ok(Json(Health {
        status: "ready".to_owned(),
    }))
}

#[utoipa::path(get, path = "/api/v1/me", operation_id = "getMe", security(("bearer" = [])),
    responses((status = 200, body = Identity), (status = 401, body = Problem), (status = 403, body = Problem), (status = 503, body = Problem)))]
pub async fn me(auth: Authenticated) -> Json<Identity> {
    Json(auth.identity)
}

#[utoipa::path(get, path = "/api/v1/items", operation_id = "listItems", params(Pagination), security(("bearer" = [])),
    responses((status = 200, body = ItemPage), (status = 400, body = Problem), (status = 401, body = Problem), (status = 403, body = Problem), (status = 503, body = Problem)))]
pub async fn list(
    auth: Authenticated,
    page: Result<Query<Pagination>, QueryRejection>,
) -> Result<Json<ItemPage>, ApiError> {
    let Query(page) = page.map_err(|_| auth.context.error(ErrorKind::Validation))?;
    if !(1..=MAX_PAGE_SIZE).contains(&page.limit) || page.offset < 0 {
        return Err(auth.context.error(ErrorKind::Validation));
    }
    items::list(&auth.state.pool, auth.identity.id, page)
        .await
        .map(Json)
        .map_err(|error| auth.context.store_error(error))
}

#[utoipa::path(post, path = "/api/v1/items", operation_id = "createItem", request_body = CreateItem, security(("bearer" = [])),
    responses((status = 201, body = Item, headers(("Location" = String, description = "Created Item path"))), (status = 400, body = Problem), (status = 401, body = Problem), (status = 403, body = Problem), (status = 413, body = Problem), (status = 503, body = Problem)))]
pub async fn create(
    auth: Authenticated,
    input: Result<Json<CreateItem>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(input) = input.map_err(|error| json_error(&auth.context, error))?;
    let title =
        normalize_title(&input.title).ok_or_else(|| auth.context.error(ErrorKind::Validation))?;
    let item = items::create(
        &auth.state.pool,
        auth.identity.id,
        CreateItem {
            title,
            completed: input.completed,
        },
    )
    .await
    .map_err(|error| auth.context.store_error(error))?;
    let location = format!("/api/v1/items/{}", item.id);
    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, location)],
        Json(item),
    )
        .into_response())
}

#[utoipa::path(get, path = "/api/v1/items/{id}", operation_id = "getItem", params(("id" = Uuid, Path)), security(("bearer" = [])),
    responses((status = 200, body = Item), (status = 400, body = Problem), (status = 401, body = Problem), (status = 403, body = Problem), (status = 404, body = Problem), (status = 503, body = Problem)))]
pub async fn get(
    auth: Authenticated,
    id: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<Item>, ApiError> {
    let Path(id) = id.map_err(|_| auth.context.error(ErrorKind::Validation))?;
    items::get(&auth.state.pool, auth.identity.id, id)
        .await
        .map(Json)
        .map_err(|error| auth.context.store_error(error))
}

#[utoipa::path(patch, path = "/api/v1/items/{id}", operation_id = "updateItem", params(("id" = Uuid, Path)), request_body = UpdateItem, security(("bearer" = [])),
    responses((status = 200, body = Item), (status = 400, body = Problem), (status = 401, body = Problem), (status = 403, body = Problem), (status = 404, body = Problem), (status = 409, body = Problem), (status = 413, body = Problem), (status = 503, body = Problem)))]
pub async fn update(
    auth: Authenticated,
    id: Result<Path<Uuid>, PathRejection>,
    input: Result<Json<UpdateItem>, JsonRejection>,
) -> Result<Json<Item>, ApiError> {
    let Path(id) = id.map_err(|_| auth.context.error(ErrorKind::Validation))?;
    let Json(input) = input.map_err(|error| json_error(&auth.context, error))?;
    if input.version < 1 || (input.title.is_none() && input.completed.is_none()) {
        return Err(auth.context.error(ErrorKind::Validation));
    }
    let title = input
        .title
        .as_deref()
        .map(|value| {
            normalize_title(value).ok_or_else(|| auth.context.error(ErrorKind::Validation))
        })
        .transpose()?;
    let command = UpdateCommand {
        id,
        input: UpdateItem { title, ..input },
    };
    items::update(&auth.state.pool, auth.identity.id, command)
        .await
        .map(Json)
        .map_err(|error| auth.context.store_error(error))
}

#[utoipa::path(delete, path = "/api/v1/items/{id}", operation_id = "deleteItem", params(("id" = Uuid, Path), DeleteVersion), security(("bearer" = [])),
    responses((status = 204, description = "Deleted"), (status = 400, body = Problem), (status = 401, body = Problem), (status = 403, body = Problem), (status = 404, body = Problem), (status = 409, body = Problem), (status = 503, body = Problem)))]
pub async fn delete(
    auth: Authenticated,
    id: Result<Path<Uuid>, PathRejection>,
    version: Result<Query<DeleteVersion>, QueryRejection>,
) -> Result<StatusCode, ApiError> {
    let Path(id) = id.map_err(|_| auth.context.error(ErrorKind::Validation))?;
    let Query(version) = version.map_err(|_| auth.context.error(ErrorKind::Validation))?;
    if version.version < 1 {
        return Err(auth.context.error(ErrorKind::Validation));
    }
    items::delete(&auth.state.pool, auth.identity.id, (id, version.version))
        .await
        .map_err(|error| auth.context.store_error(error))?;
    Ok(StatusCode::NO_CONTENT)
}
