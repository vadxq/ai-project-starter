use crate::{
    domain::models::Identity,
    platform::{auth::AuthError, db::DatabasePool},
};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

mod mysql;
mod postgres;

#[derive(FromRow)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(FromRow)]
pub struct Session {
    pub id: Uuid,
    pub account_id: Uuid,
}

pub struct NewSession {
    pub session: Session,
    pub refresh_hash: String,
    pub expires_at: DateTime<Utc>,
}

pub struct Rotation {
    pub previous_hash: String,
    pub next_hash: String,
}

pub struct AccountInput {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

pub async fn account(
    database: &DatabasePool,
    username: &str,
) -> Result<Option<Account>, AuthError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::account(pool, username).await,
        DatabasePool::MySql(pool) => mysql::account(pool, username).await,
    }
}

pub async fn insert_session(database: &DatabasePool, session: NewSession) -> Result<(), AuthError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::insert_session(pool, session).await,
        DatabasePool::MySql(pool) => mysql::insert_session(pool, session).await,
    }
}

pub async fn rotate(
    database: &DatabasePool,
    rotation: Rotation,
) -> Result<(Session, Identity), AuthError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::rotate(pool, rotation).await,
        DatabasePool::MySql(pool) => mysql::rotate(pool, rotation).await,
    }
}

pub async fn logout(database: &DatabasePool, hash: String) -> Result<(), AuthError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::logout(pool, hash).await,
        DatabasePool::MySql(pool) => mysql::logout(pool, hash).await,
    }
}

pub async fn verify(
    database: &DatabasePool,
    ids: (Uuid, Uuid),
) -> Result<Option<Identity>, AuthError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::verify(pool, ids).await,
        DatabasePool::MySql(pool) => mysql::verify(pool, ids).await,
    }
}

pub async fn create_account(database: &DatabasePool, input: AccountInput) -> Result<(), AuthError> {
    match database {
        DatabasePool::Postgres(pool) => postgres::create_account(pool, input).await,
        DatabasePool::MySql(pool) => mysql::create_account(pool, input).await,
    }
}
