use poise::serenity_prelude::User;
use crate::types::{Context, Error};

#[poise::command(
    slash_command,
    aliases("ban"),
    owners_only,
    category = "Moderation"
)]
pub async fn clean(
    ctx: Context<'_>,
    #[description = "The user to ban"]
    user: User,
    #[description = "The timestamp of messages to delete"]
    time: Option<String>,
) -> Result<(), Error> {

    Ok(())
}
