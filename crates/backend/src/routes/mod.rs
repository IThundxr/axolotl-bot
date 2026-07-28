use crate::app::App;
use axum::Router;

pub mod api;

pub fn router() -> Router<App> {
    Router::new().nest("/api", api::router())
}
