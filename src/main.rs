use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use std::env;
mod models;
mod responses;
use models::poise_required_structs::*;
use responses::basic::*;
use responses::raiderio_responses::*;

#[tokio::main]
async fn main() {
    let commands = vec![test(), progression()];
    let _ = dotenv();
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
}
