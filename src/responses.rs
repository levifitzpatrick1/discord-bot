use serenity::all::{Context, Message};
const HELP_MESSAGE: &str = "pong! = pong!";

pub async fn ping_response(ctx: Context, msg: Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
        println!("Error sending message: {why:?}")
    }
}

pub async fn help_response(ctx: Context, msg: Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
        println!("Error sending message: {why:?}")
    }
}

pub async fn test_response(ctx: Context, msg: Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "this is a test").await {
        println!("Error sending message: {why:?}")
    }
}

pub async fn do_nothing() {}
