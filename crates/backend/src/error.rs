use std::env;
use std::ffi::OsStr;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use oauth2::url;
use uuid::Uuid;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum Error {
    // Non route errors
    #[error("Missing environment variable")]
    MissingEnvVar(#[from] std::env::VarError),
    #[error("Failed to parse url")]
    UrlParse(#[from] url::ParseError),
    #[error("Database error")]
    Database(#[from] sea_orm::DbErr),

    // Route errors
}

#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: &'static str,
    reference: Uuid,
}

impl Error {
    fn as_error_code(&self) -> (&'static str, &'static str, StatusCode) {
        match self {
            // Route Errors go here
            _ => (
                "UNKNOWN_ERR",
                "An unknown error occurred",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let reference = Uuid::new_v4();
        let (code, message, status) = self.as_error_code();

        error!("[{reference}] {self:?}");

        let body = Json(ErrorResponse {
            code,
            message,
            reference,
        });

        (status, body).into_response()
    }
}