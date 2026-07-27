use axum::Router;
use crate::app::App;

pub fn router() -> Router<App> {
    Router::new()
        // .route("/login", get(login))
        // .route("/callback", get(callback))
        // .route("/logout", get(logout))
}