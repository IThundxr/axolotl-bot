use crate::commands::BotCommand;
use crate::types::{Context, Error};
use macros::register;

#[register(BotCommand)]
#[poise::command(
    prefix_command,
    slash_command,
    aliases("allowedservers"),
    owners_only,
    category = "Bot Control"
)]
pub async fn allowed_servers(ctx: Context<'_>) -> Result<(), Error> {
    let mut ids = ctx.data().allowed_guild_ids();
    if ids.is_empty() {
        ctx.reply("No servers are allowed.").await?;
        return Ok(());
    }

    ids.sort();
    let lines = ids
        .iter()
        .map(|id| match id.name(ctx.cache()) {
            Some(name) => format!("`{id}` {name}"),
            None => format!("`{id}` (not joined)"),
        })
        .collect::<Vec<_>>()
        .join("\n");

    ctx.reply(format!("**Allowed servers ({})**\n{lines}", ids.len())).await?;
    Ok(())
}
