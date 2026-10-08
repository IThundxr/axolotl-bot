use crate::app::App;
use crate::error::Error;
use crate::users::{AuthSession, AuthUserModel};
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use database_entity::allowed_guilds;
use database_entity::prelude::AllowedGuilds;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

const ADMINISTRATOR: u64 = 1 << 3;
const MANAGE_GUILD: u64 = 1 << 5;

pub fn router() -> Router<App> {
    Router::new()
        .route("/available", get(available))
        .route("/{guild_id}", get(guild))
}

/// Partial guild object returned by Discord's `GET /users/@me/guilds`
#[derive(Deserialize)]
struct DiscordGuild {
    id: String,
    owner: bool,
    permissions: String,
}

impl DiscordGuild {
    fn can_manage(&self) -> bool {
        let permissions = self.permissions.parse::<u64>().unwrap_or(0);
        self.owner || permissions & (ADMINISTRATOR | MANAGE_GUILD) != 0
    }
}

#[derive(Serialize)]
struct GuildResponse {
    // Snowflakes are serialized as strings, as they don't fit in a JS number
    id: String,
    name: String,
    icon: Option<String>,
    owner_id: String,
}

impl From<allowed_guilds::Model> for GuildResponse {
    fn from(guild: allowed_guilds::Model) -> Self {
        Self {
            id: guild.id.to_string(),
            name: guild.name,
            icon: guild.icon,
            owner_id: guild.owner_id.to_string(),
        }
    }
}

/// Fetches the IDs of the guilds the user is allowed to manage from Discord
async fn manageable_guild_ids(app: &App, user: &AuthUserModel) -> Result<Vec<i64>, Error> {
    let guilds = app
        .http
        .get("https://discord.com/api/users/@me/guilds")
        .bearer_auth(user.access_token().secret())
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<DiscordGuild>>()
        .await?;

    Ok(guilds
        .iter()
        .filter(|guild| guild.can_manage())
        .filter_map(|guild| guild.id.parse().ok())
        .collect())
}

/// Lists the allowed guilds the logged-in user can manage
async fn available(
    State(app): State<App>,
    auth_session: AuthSession,
) -> Result<Json<Vec<GuildResponse>>, Error> {
    let Some(user) = &auth_session.user else {
        return Err(Error::NotLoggedIn);
    };

    let ids = manageable_guild_ids(&app, user).await?;
    let guilds = AllowedGuilds::find()
        .filter(allowed_guilds::Column::Id.is_in(ids))
        .order_by_asc(allowed_guilds::Column::Name)
        .all(&app.db)
        .await?;

    Ok(Json(guilds.into_iter().map(Into::into).collect()))
}

/// Gets a single allowed guild, if the logged-in user can manage it
async fn guild(
    State(app): State<App>,
    auth_session: AuthSession,
    Path(guild_id): Path<i64>,
) -> Result<Json<GuildResponse>, Error> {
    let Some(user) = &auth_session.user else {
        return Err(Error::NotLoggedIn);
    };

    if !manageable_guild_ids(&app, user).await?.contains(&guild_id) {
        return Err(Error::GuildNotFound);
    }

    AllowedGuilds::find_by_id(guild_id)
        .one(&app.db)
        .await?
        .map(|guild| Json(guild.into()))
        .ok_or(Error::GuildNotFound)
}
