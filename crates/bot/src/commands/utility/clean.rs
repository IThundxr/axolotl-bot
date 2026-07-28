use crate::commands::BotCommand;
use crate::types::{Context, Error};
use macros::register;
use poise::serenity_prelude::User;

#[register(BotCommand)]
#[poise::command(
    prefix_command,
    slash_command,
    aliases("clear"),
    owners_only,
    category = "Utility"
)]
pub async fn clean(
    ctx: Context<'_>,
    #[description = "The number of messages to delete"]
    #[min = 2]
    #[max = 300]
    count: i32,
    user: Option<User>,
) -> Result<(), Error> {
    // GetMessages::new()
    //     .before();

    ctx.reply(format!("{}", count)).await?;
    Ok(())
}
