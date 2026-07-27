use axum::Router;
use crate::app::App;

pub mod auth;

pub fn router() -> Router<App> {
    Router::new()
        .nest("/auth", auth::router())
}