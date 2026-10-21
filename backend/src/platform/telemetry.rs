//! tracing 日志初始化。

use tracing_subscriber::{EnvFilter, prelude::*};

use crate::platform::config::Settings;

/// 按配置选择文本或 JSON 日志格式，并安装全局 subscriber。
pub fn init_tracing(settings: &Settings) -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_new(&settings.telemetry.log_level)?;

    let fmt_layer = tracing_subscriber::fmt::layer();

    if settings.telemetry.json {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer.json())
            .try_init()
            .map_err(anyhow::Error::msg)?;
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .try_init()
            .map_err(anyhow::Error::msg)?;
    }

    Ok(())
}
