use migration::{Migrator, MigratorTrait};
use tracing::debug;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenvy::dotenv()?;

    // Setup tracing
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let db = database::connect().await?;

    debug!("Running migrations");
    Migrator::up(&db, None).await?;
    debug!("Finished running migrations");

    tokio::select! {
        result = backend::run(db.clone()) => {
            result?
        }
        result = bot::run(db.clone()) => {
            result?
        }
    }

    Ok(())
}
