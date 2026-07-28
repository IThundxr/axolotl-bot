use crate::commands::BotCommand;
use crate::types::{Context, Error};
use macros::register;
use poise::serenity_prelude::User;

#[register(BotCommand)]
#[poise::command(prefix_command, slash_command, owners_only, category = "Moderation")]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The user to ban"] user: User,
    #[description = "The timestamp of messages to delete"] time: Option<String>,
) -> Result<(), Error> {
    Ok(())
}
