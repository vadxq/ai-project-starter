//! 业务服务仅分派强类型数据库实现；两种方言保持同一权限和事务语义。
pub mod handler;
mod mysql;
mod postgres;
pub mod routes;

use crate::{
    domain::models::{CreateItem, Item, ItemPage, Pagination, UpdateItem},
    platform::db::DatabasePool,
};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("resource not found")]
    NotFound,
    #[error("resource version conflict")]
    Conflict,
    #[error("database operation failed")]
    Database(#[from] sqlx::Error),
}

pub struct UpdateCommand {
    pub id: Uuid,
    pub input: UpdateItem,
}

pub async fn list(
    database: &DatabasePool,
    owner: Uuid,
    page: Pagination,
) -> Result<ItemPage, StoreError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::list(pool, owner, page).await,
        DatabasePool::MySql(pool) => mysql::list(pool, owner, page).await,
    }
}

pub async fn create(
    database: &DatabasePool,
    owner: Uuid,
    input: CreateItem,
) -> Result<Item, StoreError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::create(pool, owner, input).await,
        DatabasePool::MySql(pool) => mysql::create(pool, owner, input).await,
    }
}

pub async fn get(database: &DatabasePool, owner: Uuid, id: Uuid) -> Result<Item, StoreError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::get(pool, owner, id).await,
        DatabasePool::MySql(pool) => mysql::get(pool, owner, id).await,
    }
}

pub async fn update(
    database: &DatabasePool,
    owner: Uuid,
    command: UpdateCommand,
) -> Result<Item, StoreError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::update(pool, owner, command).await,
        DatabasePool::MySql(pool) => mysql::update(pool, owner, command).await,
    }
}

pub async fn delete(
    database: &DatabasePool,
    owner: Uuid,
    versioned_id: (Uuid, i32),
) -> Result<(), StoreError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::delete(pool, owner, versioned_id).await,
        DatabasePool::MySql(pool) => mysql::delete(pool, owner, versioned_id).await,
    }
}
