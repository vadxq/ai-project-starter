//! `rust_axum_starter` 的 crate 入口，按 API、domain 和 platform 分层暴露模块。

pub mod api;
pub mod bootstrap;
pub mod domain;
pub mod platform;

rust_i18n::i18n!("locales", fallback = "en");
