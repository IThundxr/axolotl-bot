use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;
use crate::app::App;
use crate::users::AuthSession;

pub fn router() -> Router<App> {
    Router::new()
        .route("/", get(status))
}

#[derive(Serialize)]
struct AuthStatus {
    logged_in: bool,
}

async fn status(auth_session: AuthSession) -> impl IntoResponse {
    Json(AuthStatus {
        logged_in: auth_session.user.is_some(),
    })
}