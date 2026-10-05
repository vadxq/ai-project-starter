use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::{CreateItem, Item, ItemPage, Pagination, UpdateItem};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("resource not found")]
    NotFound,
    #[error("resource version conflict")]
    Conflict,
    #[error("database operation failed")]
    Database(#[from] sqlx::Error),
}

pub async fn list(pool: &PgPool, owner: Uuid, page: Pagination) -> Result<ItemPage, StoreError> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
        .execute(&mut *tx)
        .await?;
    let total: i64 = sqlx::query_scalar("SELECT count(*) FROM items WHERE owner_id = $1")
        .bind(owner)
        .fetch_one(&mut *tx)
        .await?;
    let items = sqlx::query_as::<_, Item>(
        "SELECT * FROM items WHERE owner_id = $1 ORDER BY created_at DESC, id DESC LIMIT $2 OFFSET $3",
    ).bind(owner).bind(page.limit).bind(page.offset).fetch_all(&mut *tx).await?;
    tx.commit().await?;
    Ok(ItemPage {
        items,
        total,
        limit: page.limit,
        offset: page.offset,
    })
}

pub async fn create(pool: &PgPool, owner: Uuid, input: CreateItem) -> Result<Item, StoreError> {
    Ok(sqlx::query_as::<_, Item>(
        "INSERT INTO items (id, owner_id, title, completed, version) VALUES ($1, $2, $3, $4, 1) RETURNING *",
    ).bind(Uuid::new_v4()).bind(owner).bind(input.title).bind(input.completed).fetch_one(pool).await?)
}

pub async fn get(pool: &PgPool, owner: Uuid, id: Uuid) -> Result<Item, StoreError> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE owner_id = $1 AND id = $2")
        .bind(owner)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(StoreError::NotFound)
}

async fn lock_item(
    tx: &mut Transaction<'_, Postgres>,
    owner: Uuid,
    id: Uuid,
) -> Result<Item, StoreError> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE owner_id = $1 AND id = $2 FOR UPDATE")
        .bind(owner)
        .bind(id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(StoreError::NotFound)
}

pub struct UpdateCommand {
    pub id: Uuid,
    pub input: UpdateItem,
}

pub async fn update(
    pool: &PgPool,
    owner: Uuid,
    command: UpdateCommand,
) -> Result<Item, StoreError> {
    let mut tx = pool.begin().await?;
    let item = lock_item(&mut tx, owner, command.id).await?;
    if item.version != command.input.version || item.version == i32::MAX {
        return Err(StoreError::Conflict);
    }
    let updated = sqlx::query_as::<_, Item>(
        "UPDATE items SET title = $1, completed = $2, version = version + 1, \
         updated_at = clock_timestamp() WHERE id = $3 RETURNING *",
    )
    .bind(command.input.title.unwrap_or(item.title))
    .bind(command.input.completed.unwrap_or(item.completed))
    .bind(command.id)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(updated)
}

pub async fn delete(
    pool: &PgPool,
    owner: Uuid,
    versioned_id: (Uuid, i32),
) -> Result<(), StoreError> {
    let mut tx = pool.begin().await?;
    let item = lock_item(&mut tx, owner, versioned_id.0).await?;
    if item.version != versioned_id.1 {
        return Err(StoreError::Conflict);
    }
    sqlx::query("DELETE FROM items WHERE id = $1")
        .bind(item.id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
