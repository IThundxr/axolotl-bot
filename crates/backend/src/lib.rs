mod app;
mod error;
mod routes;
mod users;

use crate::app::App;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;
use axum_login::AuthManagerLayerBuilder;
use axum_login::tower_sessions::{Expiry, SessionManagerLayer};
use sea_orm::DatabaseConnection;
use std::env;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tower_sessions::cookie::SameSite;
use tower_sessions::cookie::time::Duration;
use tower_sessions_redis_store::{RedisStore, fred::prelude::*};
use tracing::{debug, info};
use crate::users::AuthSession;

pub async fn run(db: DatabaseConnection) -> anyhow::Result<()> {
    let app = App::new(db).await?;

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
        .route(
            // TODO - Temporary
            "/self",
            get(|auth_session: AuthSession| async {
                if auth_session.user.is_none() {
                    return "Not logged in".into();
                }

                format!("{:?}", auth_session.user.unwrap())
            }),
        )
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
