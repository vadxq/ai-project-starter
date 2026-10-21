//! 基础设施层：配置、数据库和运行时辅助能力。

pub mod auth;
pub mod config;
pub mod db;
pub mod telemetry;

/// 等待进程终止信号，用于 Axum 的优雅关闭。
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Linux/macOS 同时监听 Ctrl+C 与 SIGTERM，非 Unix 平台只监听 Ctrl+C。
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
