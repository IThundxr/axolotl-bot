//! Keeps the bot restricted to allowed servers, similar to Zeppelin's `GuildAccessMonitor` and
//! `GuildInfoSaver` plugins.

use crate::app::App;
use crate::types::Error;
use poise::serenity_prelude as serenity;
use serenity::{FullEvent, GuildId};
use std::env;
use tracing::{error, info};

/// Adds the servers listed in `BOT_DEFAULT_ALLOWED_SERVERS` to the allow list.
pub async fn add_default_allowed_guilds(app: &App) -> anyhow::Result<()> {
    let Ok(servers) = env::var("BOT_DEFAULT_ALLOWED_SERVERS") else {
        return Ok(());
    };

    for id in servers.split(',').map(str::trim).filter(|id| !id.is_empty()) {
        let id = GuildId::new(id.parse()?);
        if app.allow_guild(id, None, None, None).await? {
            info!("Adding allowed-by-default server {id} to the allowed servers");
        }
    }

    Ok(())
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    app: &App,
) -> Result<(), Error> {
    match event {
        // Fired both for servers the bot is already in on startup and for newly joined ones
        FullEvent::GuildCreate { guild, .. } => {
            if !app.is_allowed(guild.id) {
                info!("Non-allowed server {} ({}), leaving", guild.name, guild.id);
                if let Err(e) = guild.id.leave(&ctx.http).await {
                    error!("Failed to leave non-allowed server {}: {e}", guild.id);
                }
                return Ok(());
            }

            app.update_guild_info(
                guild.id,
                guild.name.clone(),
                guild.icon_url(),
                guild.owner_id.get() as i64,
            )
            .await?;
        },
        FullEvent::GuildUpdate { new_data, .. } => {
            app.update_guild_info(
                new_data.id,
                new_data.name.clone(),
                new_data.icon_url(),
                new_data.owner_id.get() as i64,
            )
            .await?;
        },
        _ => {},
    }

    Ok(())
}
