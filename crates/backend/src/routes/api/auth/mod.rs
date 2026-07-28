use crate::app::App;
use axum::Router;

pub mod discord;

pub fn router() -> Router<App> {
    Router::new().nest("/discord", discord::router())
}
