mod help;
mod moderation;
pub mod utility;

use crate::app::App;
use crate::types::Error;

pub struct BotCommand(pub fn() -> poise::Command<App, Error>);

inventory::collect!(BotCommand);

#[inline(always)]
pub fn routes() -> Vec<poise::Command<App, Error>> {
    inventory::iter::<BotCommand>
        .into_iter()
        .map(|cmd| cmd.0())
        .collect()
}
