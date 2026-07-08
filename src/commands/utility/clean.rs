use crate::types::{Context, Error};
use poise::serenity_prelude::{GetMessages, User};

#[poise::command(
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
    user: Option<User>
) -> Result<(), Error> {
    // GetMessages::new()
    //     .before();

    ctx.reply(format!("{}", count)).await?;
    Ok(())
}
