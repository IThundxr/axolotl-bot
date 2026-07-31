use crate::app::App;
use axum::Router;

pub mod discord;
pub mod status;

pub fn router() -> Router<App> {
    Router::new()
        .nest("/discord", discord::router())
        .nest("/status", status::router())
}
