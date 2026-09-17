use crate::error::Error;
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, RevocationUrl,
    StandardRevocableToken, TokenUrl,
};
use reqwest::Client;
use reqwest::redirect::Policy;
use sea_orm::DatabaseConnection;
use std::env;
use oauth2_reqwest::ReqwestClient;

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
    pub oauth_http: ReqwestClient,
    /// SeaORM Database Connection
    pub db: DatabaseConnection,
    /// Discord OAuth Client
    pub discord_oauth: OAuthClient,
}

impl App {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Error> {
        let app = Self {
            http: Client::new(),
            oauth_http: ReqwestClient::from(Client::builder().redirect(Policy::none()).build()?),
            db,
            discord_oauth: {
                let client_id = env::var("BACKEND_DISCORD_CLIENT_ID")?;
                let client_secret = env::var("BACKEND_DISCORD_CLIENT_SECRET")?;
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
