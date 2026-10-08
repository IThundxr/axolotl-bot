use crate::commands::BotCommand;
use crate::types::{Context, Error};
use macros::register;
use poise::serenity_prelude::GuildId;

#[register(BotCommand)]
#[poise::command(
    prefix_command,
    slash_command,
    aliases("disallowserver", "remove_server", "removeserver"),
    owners_only,
    category = "Bot Control"
)]
pub async fn disallow_server(
    ctx: Context<'_>,
    #[description = "The ID of the server to disallow"] guild_id: String,
) -> Result<(), Error> {
    let Some(guild_id) = guild_id.parse().ok().filter(|&id| id != 0).map(GuildId::new) else {
        ctx.reply("Invalid server ID!").await?;
        return Ok(());
    };

    if !ctx.data().disallow_guild(guild_id).await? {
        ctx.reply("That server is not allowed in the first place!").await?;
        return Ok(());
    }

    // Reply before leaving, in case the command was run in the server being removed
    ctx.reply("Server removed!").await?;
    let _ = guild_id.leave(ctx.http()).await;
    Ok(())
}
