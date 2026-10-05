use starter_api::platform::{config::required, database};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = dotenvy::dotenv()
        && !error.not_found()
    {
        return Err(error.into());
    }
    let pool = database::connect(&required("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&pool).await?;
    pool.close().await;
    Ok(())
}
