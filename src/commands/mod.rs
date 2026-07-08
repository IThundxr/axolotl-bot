mod help;
mod moderation;
pub mod utility;

use crate::app::App;
use crate::types::Error;

#[inline(always)]
pub fn routes() -> Vec<poise::Command<App, Error>> {
    vec![utility::clean::clean(), help::help()]
}
