use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tokio::sync::Semaphore;
use uuid::Uuid;

use crate::domain::{
    auth::{LoginRequest, TokenResponse, valid_password, valid_username},
    models::Identity,
};

pub const ACCESS_SECONDS: i64 = 15 * 60;
const SESSION_DAYS: i64 = 30;
const PASSWORD_WORKERS: usize = 4;
const JWT_ISSUER: &str = "starter-api";
const JWT_AUDIENCE: &str = "starter-clients";

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("access or refresh token invalid")]
    Invalid,
    #[error("incorrect username or password")]
    Credentials,
    #[error("invalid account input")]
    Validation,
    #[error("authentication database operation failed")]
    Database(#[from] sqlx::Error),
    #[error("authentication operation failed")]
    Internal,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    sid: Uuid,
    iss: String,
    aud: String,
    exp: i64,
    iat: i64,
}

#[derive(sqlx::FromRow)]
struct Account {
    id: Uuid,
    username: String,
    password_hash: String,
}

#[derive(sqlx::FromRow)]
struct Session {
    id: Uuid,
    account_id: Uuid,
}

pub struct TokenService {
    encoding: EncodingKey,
    decoding: DecodingKey,
    dummy_hash: String,
    passwords: Semaphore,
}

fn refresh_token() -> String {
    format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple())
}

fn token_hash(token: &str) -> Result<String, AuthError> {
    if token.len() != 64 || !token.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(AuthError::Invalid);
    }
    Ok(format!("{:x}", Sha256::digest(token.as_bytes())))
}

fn password_hash(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|value| value.to_string())
        .map_err(|_| AuthError::Internal)
}

impl TokenService {
    pub fn new(secret: &str) -> Result<Self, AuthError> {
        Ok(Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
            // 不存在的账号也执行同等 Argon2 工作，避免通过耗时枚举账号。
            dummy_hash: password_hash(&Uuid::new_v4().to_string())?,
            passwords: Semaphore::new(PASSWORD_WORKERS),
        })
    }

    fn response(
        &self,
        session: Session,
        user: Identity,
        refresh: String,
    ) -> Result<TokenResponse, AuthError> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user.id,
            sid: session.id,
            iss: JWT_ISSUER.to_owned(),
            aud: JWT_AUDIENCE.to_owned(),
            exp: now + ACCESS_SECONDS,
            iat: now,
        };
        let access = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding)
            .map_err(|_| AuthError::Internal)?;
        Ok(TokenResponse {
            access_token: access,
            refresh_token: refresh,
            token_type: "Bearer".to_owned(),
            expires_in: ACCESS_SECONDS,
            user,
        })
    }

    pub async fn login(
        &self,
        pool: &PgPool,
        input: LoginRequest,
    ) -> Result<TokenResponse, AuthError> {
        if !valid_username(&input.username)
            || input.password.is_empty()
            || input.password.len() > 128
        {
            return Err(AuthError::Validation);
        }
        let account = sqlx::query_as::<_, Account>(
            "SELECT id, username, password_hash FROM accounts WHERE username = $1",
        )
        .bind(&input.username)
        .fetch_optional(pool)
        .await?;
        let hash = account
            .as_ref()
            .map_or(&self.dummy_hash, |value| &value.password_hash)
            .clone();
        let _permit = self
            .passwords
            .acquire()
            .await
            .map_err(|_| AuthError::Internal)?;
        let verified = tokio::task::spawn_blocking(move || {
            let parsed = PasswordHash::new(&hash).map_err(|_| AuthError::Internal)?;
            match Argon2::default().verify_password(input.password.as_bytes(), &parsed) {
                Ok(()) => Ok(true),
                Err(argon2::password_hash::Error::Password) => Ok(false),
                Err(_) => Err(AuthError::Internal),
            }
        })
        .await
        .map_err(|_| AuthError::Internal)??;
        if !verified {
            return Err(AuthError::Credentials);
        }
        let account = account.ok_or(AuthError::Credentials)?;
        let session = Session {
            id: Uuid::new_v4(),
            account_id: account.id,
        };
        let refresh = refresh_token();
        let response = self.response(
            Session {
                id: session.id,
                account_id: session.account_id,
            },
            Identity {
                id: account.id,
                username: account.username,
            },
            refresh.clone(),
        )?;
        sqlx::query("INSERT INTO auth_sessions (id, account_id, refresh_hash, expires_at) VALUES ($1, $2, $3, $4)")
            .bind(session.id).bind(session.account_id).bind(token_hash(&refresh)?)
            .bind(Utc::now() + Duration::days(SESSION_DAYS)).execute(pool).await?;
        Ok(response)
    }

    pub async fn refresh(&self, pool: &PgPool, previous: &str) -> Result<TokenResponse, AuthError> {
        let hash = token_hash(previous)?;
        let next = refresh_token();
        // 行锁内单次替换散列；并发请求只能有一个成功，不自动重放旧 refresh token。
        let mut tx = pool.begin().await?;
        let session = sqlx::query_as::<_, Session>(
            "UPDATE auth_sessions SET refresh_hash = $1 WHERE refresh_hash = $2 AND revoked_at IS NULL AND expires_at > now() RETURNING id, account_id")
            .bind(token_hash(&next)?).bind(hash).fetch_optional(&mut *tx).await?.ok_or(AuthError::Invalid)?;
        let user = sqlx::query_as::<_, Identity>("SELECT id, username FROM accounts WHERE id = $1")
            .bind(session.account_id)
            .fetch_one(&mut *tx)
            .await?;
        let response = self.response(session, user, next)?;
        tx.commit().await?;
        Ok(response)
    }

    pub async fn logout(&self, pool: &PgPool, refresh: &str) -> Result<(), AuthError> {
        sqlx::query("UPDATE auth_sessions SET revoked_at = now() WHERE refresh_hash = $1 AND revoked_at IS NULL")
            .bind(token_hash(refresh)?).execute(pool).await?;
        Ok(())
    }

    pub async fn verify(&self, pool: &PgPool, token: &str) -> Result<Identity, AuthError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[JWT_ISSUER]);
        validation.set_audience(&[JWT_AUDIENCE]);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
        validation.leeway = 0;
        let claims = decode::<Claims>(token, &self.decoding, &validation)
            .map_err(|_| AuthError::Invalid)?
            .claims;
        // session 撤销后 access JWT 也立即失效，避免退出后继续访问私有数据。
        sqlx::query_as::<_, Identity>(
            "SELECT a.id, a.username FROM accounts a JOIN auth_sessions s ON s.account_id = a.id WHERE a.id = $1 AND s.id = $2 AND s.revoked_at IS NULL AND s.expires_at > now()")
            .bind(claims.sub).bind(claims.sid).fetch_optional(pool).await?.ok_or(AuthError::Invalid)
    }
}

pub async fn create_account(
    pool: &PgPool,
    username: String,
    password: String,
) -> Result<(), AuthError> {
    if !valid_username(&username) || !valid_password(&password) {
        return Err(AuthError::Validation);
    }
    let hash = tokio::task::spawn_blocking(move || password_hash(&password))
        .await
        .map_err(|_| AuthError::Internal)??;
    let id = Uuid::new_v4();
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO identities (id, issuer, subject) VALUES ($1, 'local', $2)")
        .bind(id)
        .bind(id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO accounts (id, username, password_hash) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(username)
        .bind(hash)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
