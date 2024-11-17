use auth::get_oauth_token;
use db::{db_init::init_db, db_updates::schedule_data_updates};
use dotenvy::dotenv;
use models::poise_required_structs::Data;
use poise::serenity_prelude as serenity;
use responses::{crafting_responses::craft_request, progress_responses::{character_progression, progression}, basic::test};
use std::env;

mod models;
mod responses;
mod db;
mod auth;

#[tokio::main]
async fn main() {
    let _ = dotenv();
    let _ = get_oauth_token().await;
    if let Err(err) = init_db() {
        eprint!("Error initing db: {}", err);
    }
    if let Err(err) = schedule_data_updates().await {
        eprintln!("Error in scheduled update: {}", err);
    }

    
    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    // Define commands for the bot
    let commands = vec![test(), progression(), character_progression(), craft_request()];

    // Set up the Discord bot framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { })
            })
        })
        .build();

    // Create and run the Discord client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}