use crate::models::poise_required_structs::*;
use poise::serenity_prelude as serenity;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("test").await?;
    Ok(())
}

// pub async fn test_response(ctx: Context, msg: Message) {
//     if let Err(why) = msg.channel_id.say(&ctx.http, "this is a test").await {
//         println!("Error sending message: {why:?}")
//     }
// }
