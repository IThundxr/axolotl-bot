use crate::app::App;
use crate::error::Error;
use axum_login::{AuthUser, AuthnBackend, UserId};
use database_entity::prelude::Users;
use database_entity::users;
use oauth2::{AccessToken, AuthorizationCode, PkceCodeVerifier, RefreshToken, TokenResponse};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::{ColumnTrait, QueryFilter};
use serde::Deserialize;
use uuid::Uuid;

pub type AuthSession = axum_login::AuthSession<App>;

#[derive(Deserialize)]
pub struct DiscordUser {
    id: String,
    username: String,
}

/// Wrapper over [users::Model] so that it can impl AuthUser
#[derive(Debug, Clone)]
pub struct AuthUserModel(users::Model);

pub struct Credentials {
    pub discord_code: AuthorizationCode,
    pub pkce_code_verifier: PkceCodeVerifier,
}

impl AuthUserModel {
    pub fn access_token(&self) -> AccessToken {
        AccessToken::new(self.0.discord_access_token.clone())
    }

    pub fn refresh_token(&self) -> RefreshToken {
        RefreshToken::new(self.0.discord_refresh_token.clone())
    }
}

impl AuthUser for AuthUserModel {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.0.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.0.session_secret.as_bytes()
    }
}

impl AuthnBackend for App {
    type User = AuthUserModel;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let discord_token = self
            .discord_oauth
            .exchange_code(creds.discord_code)
            .set_pkce_verifier(creds.pkce_code_verifier)
            .request_async(&self.oauth_http)
            .await?;

        let discord_user_info = self
            .http
            .get("https://discord.com/api/users/@me")
            .bearer_auth(discord_token.access_token().secret())
            .send()
            .await?
            .json::<DiscordUser>()
            .await?;

        let discord_id = discord_user_info.id.parse::<i64>()?;

        let existing_user: Result<Option<users::Model>, Self::Error> = Users::find()
            .filter(users::Column::DiscordId.eq(discord_id))
            .one(&self.db)
            .await
            .map_err(Error::Database);

        let mut active_user = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            session_secret: Set(Uuid::new_v4()),
            discord_id: Set(discord_id),
            discord_access_token: Set(discord_token.access_token().secret().clone()),
            discord_refresh_token: Set(discord_token.refresh_token().unwrap().secret().clone()),
            ..Default::default()
        };

        let user = if let Ok(Some(existing_user)) = existing_user {
            active_user.id = Set(existing_user.id);
            active_user.session_secret = Set(existing_user.session_secret);
            active_user.update(&self.db).await?
        } else {
            active_user.insert(&self.db).await?
        };

        Ok(Some(AuthUserModel(user)))
    }

    async fn get_user(&self, uuid: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Users::find()
            .filter(users::Column::Id.eq(*uuid))
            .one(&self.db)
            .await
            .map(|model| model.map(AuthUserModel))
            .map_err(Into::into)
    }
}
