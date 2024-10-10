use crate::models::poise_required_structs::*;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("test").await?;
    Ok(())
}
