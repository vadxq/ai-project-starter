use std::sync::Arc;

use starter_api::{
    AppState, api,
    platform::{auth::TokenService, config::Config, database},
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = dotenvy::dotenv()
        && !error.not_found()
    {
        return Err(error.into());
    }
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let config = Config::load()?;
    let pool = database::connect(&config.database_url).await?;
    // 迁移必须先显式执行；缺表时不能以健康进程伪装成可用业务服务。
    sqlx::query("SELECT id FROM accounts LIMIT 0")
        .execute(&pool)
        .await?;
    let tokens = Arc::new(TokenService::new(&config.jwt_secret)?);
    let state = AppState {
        pool: pool.clone(),
        tokens,
    };
    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    tracing::info!(event = "server_started", address = %config.bind_address);
    axum::serve(listener, api::router(state, config.cors_origins))
        .with_graceful_shutdown(shutdown())
        .await?;
    pool.close().await;
    Ok(())
}

async fn shutdown() {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("SIGTERM handler");
    tokio::select! {
        result = tokio::signal::ctrl_c() => result.expect("SIGINT handler"),
        _ = terminate.recv() => {},
    }
}
