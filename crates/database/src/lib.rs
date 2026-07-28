use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

pub async fn connect() -> Result<DatabaseConnection, anyhow::Error> {
    let database_url = env::var("DATABASE_URL")?;

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(20)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(true);

    Ok(Database::connect(opt).await?)
}
