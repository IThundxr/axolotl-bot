use poise::serenity_prelude as serenity;

use crate::app::App;
use crate::types::Error;
use serenity::{ClientBuilder, GatewayIntents};
use std::env;
use tracing::level_filters::LevelFilter;
use tracing::{debug, debug_span, error, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use crate::commands::BotCommand;

mod app;
mod commands;
mod config;
mod error;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env
    dotenvy::dotenv()?;

    // Setup tracing
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    // Setup serenity/poise
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::routes(),
            prefix_options: poise::PrefixFrameworkOptions {
                dynamic_prefix: Some(|ctx| Box::pin(async move {
                    Ok(match ctx.guild_id {
                        Some(id) => {
                            let span = debug_span!("dynamic_prefix", guild_id = id.to_string());
                            let _enter = span.enter();
                            let config = ctx.data.get_config(id);
                            let prefix = config.prefix.clone();
                            debug!("Prefix: {:?}", prefix);
                            Some(prefix)
                        }
                        None => None
                    })
                })),
                ..Default::default()
            },
            on_error: |ctx| Box::pin(on_error(ctx)),
            pre_command: |ctx| {
                Box::pin(async move {
                    debug!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    debug!("Executed command {}...", ctx.command().qualified_name);
                })
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!(
                    "Logged in as {}#{:?}",
                    ready.user.name,
                    ready.user.discriminator.unwrap()
                );

                info!("Registering slash commands globally");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Registered slash commands globally");
                Ok(App::new())
            })
        })
        .build();

    let token = env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client?.start().await?;

    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, App, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            let error_id = Uuid::new_v4().to_string();

            let _ = ctx.reply(format!(
                "Oops! Something went wrong while running this command. Please report this to the bot owner with error ID `{}`.",
                error_id
            )).await;

            let span = tracing::info_span!(
                "command_error",
                error_id = %error_id,
                command = %ctx.command().name,
                user = %ctx.author().tag(),
                user_id = %ctx.author().id,
                guild = ?ctx.guild_id().map_or("DM".to_string(), |id| id.to_string())
            );

            let _enter = span.enter();
            error!("Error in command execution: {error:?}");
        },
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {e}")
            }
        },
    }
}