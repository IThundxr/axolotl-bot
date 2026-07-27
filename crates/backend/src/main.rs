mod app;
mod error;
mod routes;
mod users;

use std::env;
use std::net::SocketAddr;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use axum_login::AuthManagerLayerBuilder;
use tower_sessions::cookie::SameSite;
use tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::{Expiry, SessionManagerLayer};
use tower_http::trace::TraceLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::*};
use tracing::{debug, info};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use migration::{Migrator, MigratorTrait};
use crate::app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenvy::dotenv()?;

    // Setup tracing
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let app = App::new().await?;

    debug!("Running migrations");
    Migrator::up(&app.db, None).await?;
    debug!("Finished running migrations");

    let redis_url = Config::from_url(&env::var("BACKEND_REDIS_URL")?)?;
    let redis_pool = Pool::new(redis_url, None, None, None, 6)?;

    debug!("Trying to connect to redis");
    let redis_conn = redis_pool.connect();
    redis_pool.wait_for_connect().await?;
    debug!("Connected to redis");

    let session_store = RedisStore::new(redis_pool);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(cfg!(not(debug_assertions)))
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    let auth_layer = AuthManagerLayerBuilder::new(app.clone(), session_layer).build();

    let router = Router::new()
        .route("/status", get(|| async { StatusCode::OK }))
        .merge(routes::router())
        .layer(TraceLayer::new_for_http())
        .layer(auth_layer)
        .with_state(app);

    let ip = env::var("APP_IP").unwrap_or("0.0.0.0".to_string());
    let port = env::var("APP_PORT").unwrap_or("3000".to_string());
    let address = format!("{ip}:{port}");

    info!("Listening on {address}");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await?;

    redis_conn.await??;

    Ok(())
}
