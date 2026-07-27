use axum::Router;
use crate::app::App;

pub mod discord;

pub fn router() -> Router<App> {
    Router::new().nest("/discord", discord::router())
}