use crate::app::App;
use axum::Router;

pub mod auth;

pub fn router() -> Router<App> {
    Router::new().nest("/auth", auth::router())
}
