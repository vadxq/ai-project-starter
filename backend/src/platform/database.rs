use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use super::config::{CONNECT_ATTEMPTS, IO_TIMEOUT};

pub async fn connect(url: &str) -> Result<PgPool, sqlx::Error> {
    for attempt in 1..=CONNECT_ATTEMPTS {
        let result = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(IO_TIMEOUT)
            .connect(url)
            .await;
        match result {
            Ok(pool) => return Ok(pool),
            Err(error) if attempt == CONNECT_ATTEMPTS => return Err(error),
            Err(_) => {
                tracing::warn!(event = "database_connect_retry", attempt);
                tokio::time::sleep(Duration::from_millis(250 * attempt as u64)).await;
            }
        }
    }
    unreachable!("non-zero connection attempts")
}
