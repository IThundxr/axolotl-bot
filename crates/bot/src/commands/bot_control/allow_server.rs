use crate::commands::BotCommand;
use crate::types::{Context, Error};
use macros::register;
use poise::serenity_prelude::GuildId;

#[register(BotCommand)]
#[poise::command(
    prefix_command,
    slash_command,
    aliases("allowserver", "add_server", "addserver"),
    owners_only,
    category = "Bot Control"
)]
pub async fn allow_server(
    ctx: Context<'_>,
    #[description = "The ID of the server to allow"] guild_id: String,
) -> Result<(), Error> {
    let Some(guild_id) = guild_id.parse().ok().filter(|&id| id != 0).map(GuildId::new) else {
        ctx.reply("Invalid server ID!").await?;
        return Ok(());
    };

    // Fill in the server info straight away if the bot can already see it
    let (name, icon, owner_id) = match guild_id.to_guild_cached(ctx.cache()) {
        Some(guild) => (
            Some(guild.name.clone()),
            guild.icon_url(),
            Some(guild.owner_id.get() as i64),
        ),
        None => (None, None, None),
    };

    if !ctx.data().allow_guild(guild_id, name, icon, owner_id).await? {
        ctx.reply("Server is already allowed!").await?;
        return Ok(());
    }

    ctx.reply("Server is now allowed to use the bot!").await?;
    Ok(())
}
