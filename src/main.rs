use dotenvy::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

mod responses;
use responses::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "ping!" => ping_response(ctx, msg).await,
            "help!" => help_response(ctx, msg).await,
            "Test!" => test_response(ctx, msg).await,
            _ => do_nothing().await,
        }
    }
}

#[tokio::main]
async fn main() {
    let _ = dotenv();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("error creating client");

    if let Err(why) = client.start().await {
        println!("client error {why:?}")
    }
}
