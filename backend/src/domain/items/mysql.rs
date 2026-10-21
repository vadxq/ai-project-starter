//! MySQL 不依赖 RETURNING；写入和读取在同一事务中完成。
use super::{StoreError, UpdateCommand};
use crate::domain::models::{CreateItem, Item, ItemPage, Pagination};
use sqlx::{Connection, MySql, MySqlPool, Transaction};
use uuid::Uuid;

pub async fn list(pool: &MySqlPool, owner: Uuid, page: Pagination) -> Result<ItemPage, StoreError> {
    let mut connection = pool.acquire().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(&mut *connection)
        .await?;
    let mut tx = connection.begin().await?;
    let total: i64 = sqlx::query_scalar("SELECT count(*) FROM items WHERE owner_id = ?")
        .bind(owner)
        .fetch_one(&mut *tx)
        .await?;
    let items = sqlx::query_as::<_, Item>(
        "SELECT * FROM items WHERE owner_id = ? ORDER BY created_at DESC, id DESC LIMIT ? OFFSET ?",
    )
    .bind(owner)
    .bind(page.limit)
    .bind(page.offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(ItemPage {
        items,
        total,
        limit: page.limit,
        offset: page.offset,
    })
}

pub async fn create(pool: &MySqlPool, owner: Uuid, input: CreateItem) -> Result<Item, StoreError> {
    let id = Uuid::new_v4();
    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT INTO items (id, owner_id, title, completed, version) VALUES (?, ?, ?, ?, 1)",
    )
    .bind(id)
    .bind(owner)
    .bind(input.title)
    .bind(input.completed)
    .execute(&mut *tx)
    .await?;
    let item = lock_item(&mut tx, owner, id).await?;
    tx.commit().await?;
    Ok(item)
}

pub async fn get(pool: &MySqlPool, owner: Uuid, id: Uuid) -> Result<Item, StoreError> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE owner_id = ? AND id = ?")
        .bind(owner)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(StoreError::NotFound)
}

async fn lock_item(
    tx: &mut Transaction<'_, MySql>,
    owner: Uuid,
    id: Uuid,
) -> Result<Item, StoreError> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE owner_id = ? AND id = ? FOR UPDATE")
        .bind(owner)
        .bind(id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(StoreError::NotFound)
}

pub async fn update(
    pool: &MySqlPool,
    owner: Uuid,
    command: UpdateCommand,
) -> Result<Item, StoreError> {
    let mut tx = pool.begin().await?;
    let item = lock_item(&mut tx, owner, command.id).await?;
    if item.version != command.input.version || item.version == i32::MAX {
        return Err(StoreError::Conflict);
    }
    sqlx::query("UPDATE items SET title = ?, completed = ?, version = version + 1, updated_at = UTC_TIMESTAMP(3) WHERE id = ?")
        .bind(command.input.title.unwrap_or(item.title))
        .bind(command.input.completed.unwrap_or(item.completed))
        .bind(command.id).execute(&mut *tx).await?;
    let updated = lock_item(&mut tx, owner, command.id).await?;
    tx.commit().await?;
    Ok(updated)
}

pub async fn delete(
    pool: &MySqlPool,
    owner: Uuid,
    versioned_id: (Uuid, i32),
) -> Result<(), StoreError> {
    let mut tx = pool.begin().await?;
    let item = lock_item(&mut tx, owner, versioned_id.0).await?;
    if item.version != versioned_id.1 {
        return Err(StoreError::Conflict);
    }
    sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(item.id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
