use auth::get_oath_token;
use db::db_init::init_db;
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use responses::crafting_responses::craft_request;
use std::env;
mod models;
mod responses;
mod db;
mod auth;
use models::poise_required_structs::*;
use responses::basic::*;
use responses::progress_responses::*;
use db::db_updates::schedule_data_updates;

#[tokio::main]
async fn main() {
    let _ = init_db();
    let commands = vec![test(), progression(), character_progression(), craft_request()];
    let _ = dotenv();
    let _ = get_oath_token();
    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

    if let Err(err) = schedule_data_updates().await {
        eprintln!("Error in scheduled update: {}", err);
    }
}
