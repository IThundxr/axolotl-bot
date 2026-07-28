use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use oauth2::{url, HttpClientError, RequestTokenError, StandardErrorResponse};
use oauth2::basic::{BasicErrorResponse, BasicErrorResponseType, BasicRevocationErrorResponse};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Error {
    // Non route errors
    #[error("Missing environment variable")]
    MissingEnvVar(#[from] std::env::VarError),
    #[error("Failed to parse url")]
    UrlParse(#[from] url::ParseError),
    #[error("Database error")]
    Database(#[from] sea_orm::DbErr),
    #[error("Session error")]
    Session(#[from] tower_sessions::session::Error),
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to exchange OAuth code")]
    OAuthExchange(#[from] RequestTokenError<HttpClientError<reqwest::Error>, BasicErrorResponse>),
    #[error("Failed to parse as int")]
    ParseIntError(#[from] std::num::ParseIntError),

    // Route errors
    #[error("Failed to authenticate user")]
    FailedToAuthenticate,
    #[error("Failed to find a valid session")]
    FailedToFindSession,
    #[error("Failed to log out")]
    FailedToLogOut,
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
            Error::FailedToAuthenticate => (
                "ERR_AUTHENTICATE",
                "Failed to authenticate user",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Error::FailedToFindSession => (
                "ERR_NO_USER_SESSION",
                "Failed to find user session",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Error::FailedToLogOut => (
                "ERR_LOGOUT",
                "Failed to log out",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
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
