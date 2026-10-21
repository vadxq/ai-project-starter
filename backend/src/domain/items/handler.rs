use crate::{
    api::{
        context::Authenticated,
        error::{AppError, ErrorResponse},
        response::{self, ApiJson, AppJson, SuccessResponse},
    },
    domain::{
        items::{self, UpdateCommand},
        models::*,
    },
};
use axum::{
    Json,
    extract::{
        Path, Query,
        rejection::{PathRejection, QueryRejection},
    },
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

#[utoipa::path(get, path = "/api/v1/me", tag = "items", operation_id = "getMe", security(("bearer" = [])),
    responses((status = 200, body = SuccessResponse<Identity>), (status = 401, body = ErrorResponse), (status = 403, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn me(auth: Authenticated) -> ApiJson<Identity> {
    response::ok(auth.identity)
}

#[utoipa::path(get, path = "/api/v1/items", tag = "items", operation_id = "listItems", params(Pagination), security(("bearer" = [])),
    responses((status = 200, body = SuccessResponse<ItemPage>), (status = 400, body = ErrorResponse), (status = 401, body = ErrorResponse), (status = 403, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn list(
    auth: Authenticated,
    page: Result<Query<Pagination>, QueryRejection>,
) -> Result<ApiJson<ItemPage>, AppError> {
    let Query(page) = page.map_err(|_| {
        auth.context
            .error(StatusCode::BAD_REQUEST, "validation_error")
    })?;
    if !(1..=MAX_PAGE_SIZE).contains(&page.limit) || page.offset < 0 {
        return Err(auth
            .context
            .error(StatusCode::BAD_REQUEST, "validation_error"));
    }
    items::list(&auth.database, auth.identity.id, page)
        .await
        .map(response::ok)
        .map_err(|error| auth.context.store_error(error))
}

#[utoipa::path(post, path = "/api/v1/items", tag = "items", operation_id = "createItem", request_body = CreateItem, security(("bearer" = [])),
    responses((status = 201, body = SuccessResponse<Item>, headers(("Location" = String, description = "Created Item path"))), (status = 400, body = ErrorResponse), (status = 401, body = ErrorResponse), (status = 403, body = ErrorResponse), (status = 413, body = ErrorResponse), (status = 415, body = ErrorResponse), (status = 422, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn create(
    auth: Authenticated,
    AppJson(input): AppJson<CreateItem>,
) -> Result<Response, AppError> {
    let title = normalize_title(&input.title).ok_or_else(|| {
        auth.context
            .error(StatusCode::BAD_REQUEST, "validation_error")
    })?;
    let item = items::create(
        &auth.database,
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
        Json(SuccessResponse::created(item)),
    )
        .into_response())
}

#[utoipa::path(get, path = "/api/v1/items/{id}", tag = "items", operation_id = "getItem", params(("id" = Uuid, Path)), security(("bearer" = [])),
    responses((status = 200, body = SuccessResponse<Item>), (status = 400, body = ErrorResponse), (status = 401, body = ErrorResponse), (status = 403, body = ErrorResponse), (status = 404, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn get(
    auth: Authenticated,
    id: Result<Path<Uuid>, PathRejection>,
) -> Result<ApiJson<Item>, AppError> {
    let Path(id) = id.map_err(|_| {
        auth.context
            .error(StatusCode::BAD_REQUEST, "validation_error")
    })?;
    items::get(&auth.database, auth.identity.id, id)
        .await
        .map(response::ok)
        .map_err(|error| auth.context.store_error(error))
}

#[utoipa::path(patch, path = "/api/v1/items/{id}", tag = "items", operation_id = "updateItem", params(("id" = Uuid, Path)), request_body = UpdateItem, security(("bearer" = [])),
    responses((status = 200, body = SuccessResponse<Item>), (status = 400, body = ErrorResponse), (status = 401, body = ErrorResponse), (status = 403, body = ErrorResponse), (status = 404, body = ErrorResponse), (status = 409, body = ErrorResponse), (status = 413, body = ErrorResponse), (status = 415, body = ErrorResponse), (status = 422, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn update(
    auth: Authenticated,
    input: (Result<Path<Uuid>, PathRejection>, AppJson<UpdateItem>),
) -> Result<ApiJson<Item>, AppError> {
    let (id, AppJson(input)) = input;
    let Path(id) = id.map_err(|_| {
        auth.context
            .error(StatusCode::BAD_REQUEST, "validation_error")
    })?;
    if input.version < 1 || (input.title.is_none() && input.completed.is_none()) {
        return Err(auth
            .context
            .error(StatusCode::BAD_REQUEST, "validation_error"));
    }
    let title = input
        .title
        .as_deref()
        .map(|value| {
            normalize_title(value).ok_or_else(|| {
                auth.context
                    .error(StatusCode::BAD_REQUEST, "validation_error")
            })
        })
        .transpose()?;
    let command = UpdateCommand {
        id,
        input: UpdateItem { title, ..input },
    };
    items::update(&auth.database, auth.identity.id, command)
        .await
        .map(response::ok)
        .map_err(|error| auth.context.store_error(error))
}

#[utoipa::path(delete, path = "/api/v1/items/{id}", tag = "items", operation_id = "deleteItem", params(("id" = Uuid, Path), DeleteVersion), security(("bearer" = [])),
    responses((status = 204, description = "Deleted"), (status = 400, body = ErrorResponse), (status = 401, body = ErrorResponse), (status = 403, body = ErrorResponse), (status = 404, body = ErrorResponse), (status = 409, body = ErrorResponse), (status = 503, body = ErrorResponse)))]
pub async fn delete(
    auth: Authenticated,
    id: Result<Path<Uuid>, PathRejection>,
    version: Result<Query<DeleteVersion>, QueryRejection>,
) -> Result<StatusCode, AppError> {
    let Path(id) = id.map_err(|_| {
        auth.context
            .error(StatusCode::BAD_REQUEST, "validation_error")
    })?;
    let Query(version) = version.map_err(|_| {
        auth.context
            .error(StatusCode::BAD_REQUEST, "validation_error")
    })?;
    if version.version < 1 {
        return Err(auth
            .context
            .error(StatusCode::BAD_REQUEST, "validation_error"));
    }
    items::delete(&auth.database, auth.identity.id, (id, version.version))
        .await
        .map_err(|error| auth.context.store_error(error))?;
    Ok(StatusCode::NO_CONTENT)
}
