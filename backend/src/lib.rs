pub mod api;
pub mod domain;
pub mod platform;

rust_i18n::i18n!("locales", fallback = "en");

use std::sync::Arc;

use platform::auth::TokenService;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub tokens: Arc<TokenService>,
}
