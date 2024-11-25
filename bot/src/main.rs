use dotenvy::dotenv;
use models::poise_required_structs::Data;
use poise::serenity_prelude as serenity;
use responses::{basic::test, crafting_responses::craft_request, progress_responses::{character_progression, progression, update_character}};
use std::env;

mod models;
mod responses;

#[tokio::main]
async fn main() {
    let _ = dotenv();
    
    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    // Define commands for the bot
    let commands = vec![progression(), character_progression(), craft_request(), update_character()];

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

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}