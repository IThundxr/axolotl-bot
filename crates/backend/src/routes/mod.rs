use axum::Router;
use crate::app::App;

pub mod api;

pub fn router() -> Router<App> {
    Router::new()
        .nest("/api", api::router())
}