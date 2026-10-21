use rust_axum_starter::platform::{config::Settings, telemetry::init_tracing};
use rust_axum_starter::{api, bootstrap::build_state, platform};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 启动顺序固定为：加载配置、初始化日志、组装状态、启动 HTTP 服务。
    let settings = Settings::load()?;
    init_tracing(&settings)?;

    let state = build_state(&settings)?;
    let app = api::router::router(state.clone());

    // 监听完成后统一接入优雅退出信号，避免中断正在处理的请求。
    let listener = tokio::net::TcpListener::bind(settings.http.bind_addr).await?;
    info!(bind_addr = %listener.local_addr()?, "api listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(platform::shutdown_signal())
        .await?;

    if let Some(database) = state.health.database {
        database.close().await;
    }

    Ok(())
}
