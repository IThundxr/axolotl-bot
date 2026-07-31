use crate::app::App;
use crate::error::Error;
use crate::error::Error::FailedToAuthenticate;
use crate::users::{AuthSession, Credentials};
use axum::Router;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use constant_time_eq::constant_time_eq;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, StandardRevocableToken};
use serde::Deserialize;
use tower_sessions_core::Session;

pub const AUTH_CSRF_TOKEN: &str = "auth_csrf_token";
pub const AUTH_PKCE_CODE_VERIFIER: &str = "auth_pkce_code_verifier";

pub fn router() -> Router<App> {
    Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/logout", post(logout))
}

#[derive(Deserialize, Debug)]
struct AuthRequest {
    code: String,
    state: String,
}

async fn login(State(app): State<App>, session: Session) -> Result<impl IntoResponse, Error> {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = app
        .discord_oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".into()))
        .add_scope(Scope::new("email".into()))
        .add_scope(Scope::new("guilds".into()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    session.insert(AUTH_CSRF_TOKEN, csrf_token.secret()).await?;
    session
        .insert(AUTH_PKCE_CODE_VERIFIER, pkce_code_verifier.secret())
        .await?;

    Ok(Redirect::to(auth_url.as_str()))
}

async fn callback(
    mut auth_session: AuthSession,
    session: Session,
    Query(query): Query<AuthRequest>,
) -> Result<impl IntoResponse, Error> {
    let code = query.code;
    let state = query.state;
    let stored_csrf_token = session.remove::<CsrfToken>(AUTH_CSRF_TOKEN).await;
    let stored_pkce_code_verifier = session.remove::<PkceCodeVerifier>(AUTH_PKCE_CODE_VERIFIER).await;

    let (Ok(Some(csrf_token)), Ok(Some(pkce_code_verifier))) =
        (stored_csrf_token, stored_pkce_code_verifier)
    else {
        return Ok((StatusCode::BAD_REQUEST, "Failed to find session data.").into_response());
    };

    if !constant_time_eq(csrf_token.secret().as_bytes(), state.as_bytes()) {
        return Ok((
            StatusCode::BAD_REQUEST,
            "CSRF token does not match the passed state.",
        )
            .into_response());
    }

    let creds = Credentials {
        discord_code: AuthorizationCode::new(code),
        pkce_code_verifier,
    };

    let Ok(Some(user)) = auth_session.authenticate(creds).await else {
        return Err(FailedToAuthenticate)
    };

    if auth_session.login(&user).await.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    Ok(Redirect::to("/").into_response())
}

async fn logout(
    State(app): State<App>,
    mut auth_session: AuthSession,
) -> Result<impl IntoResponse, Error> {
    let Some(user) = &auth_session.user else {
        return Err(Error::FailedToFindSession);
    };

    app.discord_oauth
        .revoke_token(StandardRevocableToken::RefreshToken(user.refresh_token()))
        .unwrap()
        .request_async(&app.oauth_http)
        .await
        .map_err(|_| Error::FailedToLogOut)?;

    match auth_session.logout().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(Error::FailedToLogOut),
    }
}
