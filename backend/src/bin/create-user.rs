use starter_api::platform::{auth::create_account, config::required, database};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = dotenvy::dotenv()
        && !error.not_found()
    {
        return Err(error.into());
    }
    let username = std::env::args()
        .nth(1)
        .ok_or("usage: create-user <username>; set STARTER_USER_PASSWORD")?;
    let password = required("STARTER_USER_PASSWORD")?;
    let pool = database::connect(&required("DATABASE_URL")?).await?;
    create_account(&pool, username, password).await?;
    pool.close().await;
    println!("Account created");
    Ok(())
}
