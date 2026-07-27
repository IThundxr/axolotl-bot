use std::env;
use std::time::Duration;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, RevocationUrl, StandardRevocableToken, TokenUrl};
use reqwest::Client;
use reqwest::redirect::Policy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::error::Error;

pub type OAuthClient = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
    EndpointSet,
>;

#[derive(Clone, Debug)]
pub struct App {
    /// Normal Reqwest HTTP Client
    pub http: Client,
    /// Reqwest HTTP Client with no Redirect policy set to None
    pub oauth_http: Client,
    /// SeaORM Database Connection
    pub db: DatabaseConnection,
    /// Discord OAuth Client
    pub discord_oauth: OAuthClient,
}

impl App {
    pub async fn new() -> Result<Self, Error> {
        let app = Self {
            http: Client::new(),
            oauth_http: Client::builder().redirect(Policy::none()).build().unwrap(),
            db: {
                let database_url = env::var("BACKEND_DATABASE_URL")?;

                let mut opt = ConnectOptions::new(database_url);
                let duration = Duration::from_secs(8);
                opt.max_connections(100)
                    .min_connections(5)
                    .connect_timeout(duration)
                    .acquire_timeout(duration)
                    .idle_timeout(duration)
                    .max_lifetime(duration)
                    .sqlx_logging(true);

                Database::connect(opt).await?
            },
            discord_oauth: {
                let client_id = env::var("BACKEND_DISCORD_CLIENT_ID")?;
                let client_secret =
                    env::var("BACKEND_DISCORD_CLIENT_SECRET")?;
                let auth_url = AuthUrl::new("https://discord.com/oauth2/authorize".into())?;
                let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".into())?;
                let revocation_url =
                    RevocationUrl::new("https://discord.com/api/oauth2/token/revoke".into())?;

                let base_url = env::var("BACKEND_BASE_URL")?;
                let redirect_url =
                    RedirectUrl::new(format!("{base_url}/api/auth/discord/callback"))?;

                BasicClient::new(ClientId::new(client_id))
                    .set_client_secret(ClientSecret::new(client_secret))
                    .set_auth_uri(auth_url)
                    .set_token_uri(token_url)
                    .set_revocation_url(revocation_url)
                    .set_redirect_uri(redirect_url)
            },
        };

        Ok(app)
    }
}