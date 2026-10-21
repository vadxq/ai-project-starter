use std::collections::HashMap;

use rust_axum_starter::{bootstrap::build_state, platform::config::Settings};

fn source(values: &[(&str, &str)]) -> HashMap<String, String> {
    values
        .iter()
        .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
        .collect()
}

#[test]
fn missing_configuration_uses_defaults_without_database() {
    let settings = Settings::from_sources(HashMap::new(), HashMap::new()).unwrap();
    assert_eq!(settings.http.bind_addr.to_string(), "0.0.0.0:8080");
    assert!(settings.database.url.is_none());
    assert_eq!(settings.database.max_connections.get(), 10);
    assert_eq!(settings.database.timeout_secs.get(), 3);
    assert_eq!(settings.telemetry.log_level, "info");
    assert!(!settings.telemetry.json);
}

#[test]
fn process_environment_overrides_dotenv_without_mutation() {
    let settings = Settings::from_sources(
        source(&[
            ("APP_HTTP__BIND_ADDR", "127.0.0.1:8081"),
            ("APP_TELEMETRY__JSON", "false"),
        ]),
        source(&[
            ("APP_HTTP__BIND_ADDR", " 127.0.0.1:9090 "),
            ("APP_TELEMETRY__JSON", "true"),
            ("APP_DATABASE__MAX_CONNECTIONS", "20"),
            ("APP_DATABASE__TIMEOUT_SECS", "5"),
        ]),
    )
    .unwrap();
    assert_eq!(settings.http.bind_addr.to_string(), "127.0.0.1:9090");
    assert!(settings.telemetry.json);
    assert_eq!(settings.database.max_connections.get(), 20);
    assert_eq!(settings.database.timeout_secs.get(), 5);
}

#[test]
fn empty_process_database_url_disables_dotenv_database() {
    let settings = Settings::from_sources(
        source(&[("APP_DATABASE__URL", "postgres://localhost/starter")]),
        source(&[("APP_DATABASE__URL", " ")]),
    )
    .unwrap();
    assert!(settings.database.url.is_none());
}

#[test]
fn invalid_configuration_is_rejected_before_startup() {
    for (key, value) in [
        ("APP_HTTP__BIND_ADDR", "not-an-address"),
        ("APP_DATABASE__MAX_CONNECTIONS", "0"),
        ("APP_DATABASE__MAX_CONNECTIONS", "-1"),
        ("APP_DATABASE__TIMEOUT_SECS", "0"),
        ("APP_TELEMETRY__JSON", "not-a-bool"),
        ("APP_TELEMETRY__LOG_LEVEL", "["),
        ("APP_DATABASE__TYPO", "10"),
        ("APP_AUTH__JWT_SECRET", "short"),
    ] {
        assert!(
            Settings::from_sources(HashMap::new(), source(&[(key, value)])).is_err(),
            "{key}"
        );
    }
}

#[test]
fn rust_log_overrides_app_filter() {
    let settings = Settings::from_sources(
        HashMap::new(),
        source(&[
            ("APP_TELEMETRY__LOG_LEVEL", "info"),
            ("RUST_LOG", "warn,tower_http=debug"),
        ]),
    )
    .unwrap();
    assert_eq!(settings.telemetry.log_level, "warn,tower_http=debug");
}

#[test]
fn unrelated_app_environment_does_not_prevent_startup() {
    let settings = Settings::from_sources(
        HashMap::new(),
        source(&[("APP_OTHER_TOOL_VALUE", "external")]),
    )
    .unwrap();
    assert_eq!(settings.http.bind_addr.port(), 8080);
}

#[tokio::test]
async fn database_url_selects_lazy_pool_without_a_server() {
    for (url, kind) in [
        ("postgres://localhost:1/starter", "postgres"),
        ("postgresql://localhost:1/starter", "postgres"),
        ("mysql://localhost:1/starter", "mysql"),
    ] {
        let settings =
            Settings::from_sources(HashMap::new(), source(&[("APP_DATABASE__URL", url)])).unwrap();
        let state = build_state(&settings).unwrap();
        let pool = state.health.database.unwrap();
        assert_eq!(pool.kind(), kind);
        pool.close().await;
    }
}

#[tokio::test]
async fn invalid_database_url_does_not_disclose_credentials() {
    for url in [
        "sqlite://secret-password",
        "postgres://user:secret-password@localhost:bad/db",
        "mysql://user:secret-password@localhost:bad/db",
    ] {
        let settings =
            Settings::from_sources(HashMap::new(), source(&[("APP_DATABASE__URL", url)])).unwrap();
        let error = build_state(&settings).err().expect("必须拒绝非法连接地址");
        assert!(!error.to_string().contains("secret-password"));
        assert!(error.to_string().contains("APP_DATABASE__URL"));
    }
}

#[test]
fn malformed_dotenv_fails_without_disclosing_its_contents() {
    let directory = std::env::current_dir()
        .unwrap()
        .join(".ailog")
        .join(format!("dotenv-test-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(
        directory.join(".env"),
        "APP_DATABASE__URL='secret-without-closing-quote",
    )
    .unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_rust-axum-starter"))
        .env_clear()
        .env("APP_HTTP__BIND_ADDR", "invalid")
        .current_dir(&directory)
        .output()
        .unwrap();
    std::fs::remove_dir_all(&directory).unwrap();
    assert!(!output.status.success());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains(".env"));
    assert!(!error.contains("secret-without-closing-quote"));
}
