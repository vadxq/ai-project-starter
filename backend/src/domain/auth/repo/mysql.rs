//! MySQL 会话写入使用事务和行锁，避免并发刷新重放。
use super::{Account, AccountInput, NewSession, Rotation, Session};
use crate::{domain::models::Identity, platform::auth::AuthError};
use sqlx::MySqlPool;

pub async fn account(pool: &MySqlPool, username: &str) -> Result<Option<Account>, AuthError> {
    Ok(sqlx::query_as::<_, Account>(
        "SELECT id, username, password_hash FROM accounts WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?)
}

pub async fn insert_session(pool: &MySqlPool, input: NewSession) -> Result<(), AuthError> {
    sqlx::query(
        "INSERT INTO auth_sessions (id, account_id, refresh_hash, expires_at) VALUES (?, ?, ?, ?)",
    )
    .bind(input.session.id)
    .bind(input.session.account_id)
    .bind(input.refresh_hash)
    .bind(input.expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn rotate(
    pool: &MySqlPool,
    rotation: Rotation,
) -> Result<(Session, Identity), AuthError> {
    let mut tx = pool.begin().await?;
    // 用锁定读重新检查旧 hash；等待中的并发请求在提交后无法再次消费旧凭证。
    let session = sqlx::query_as::<_, Session>(
        "SELECT id, account_id FROM auth_sessions WHERE refresh_hash = ? AND revoked_at IS NULL AND expires_at > CURRENT_TIMESTAMP FOR UPDATE"
    ).bind(rotation.previous_hash).fetch_optional(&mut *tx).await?.ok_or(AuthError::Invalid)?;
    sqlx::query("UPDATE auth_sessions SET refresh_hash = ? WHERE id = ?")
        .bind(rotation.next_hash)
        .bind(session.id)
        .execute(&mut *tx)
        .await?;
    let user = sqlx::query_as::<_, Identity>("SELECT id, username FROM accounts WHERE id = ?")
        .bind(session.account_id)
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok((session, user))
}

pub async fn logout(pool: &MySqlPool, hash: String) -> Result<(), AuthError> {
    sqlx::query("UPDATE auth_sessions SET revoked_at = CURRENT_TIMESTAMP WHERE refresh_hash = ? AND revoked_at IS NULL")
        .bind(hash).execute(pool).await?;
    Ok(())
}

pub async fn verify(
    pool: &MySqlPool,
    ids: (uuid::Uuid, uuid::Uuid),
) -> Result<Option<Identity>, AuthError> {
    Ok(sqlx::query_as::<_, Identity>(
        "SELECT a.id, a.username FROM accounts a JOIN auth_sessions s ON s.account_id = a.id WHERE a.id = ? AND s.id = ? AND s.revoked_at IS NULL AND s.expires_at > CURRENT_TIMESTAMP"
    ).bind(ids.0).bind(ids.1).fetch_optional(pool).await?)
}

pub async fn create_account(pool: &MySqlPool, input: AccountInput) -> Result<(), AuthError> {
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO identities (id, issuer, subject) VALUES (?, 'local', ?)")
        .bind(input.id)
        .bind(input.id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO accounts (id, username, password_hash) VALUES (?, ?, ?)")
        .bind(input.id)
        .bind(input.username)
        .bind(input.password_hash)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
