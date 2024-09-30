use crate::models::poise_required_structs::{Context, Error as poise_Error};
use crate::models::raiderio_structs::*;
use reqwest::Error as reqwest_error;

#[poise::command(slash_command, prefix_command)]
pub async fn progression(ctx: Context<'_>) -> Result<(), poise_Error> {
    let message = raid_progress_message().await.unwrap();
    ctx.say(message).await?;
    Ok(())
}

// pub async fn progress_response(ctx: Context, msg: Message) {
//     let message = raid_progress_message().await.unwrap();
//     if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
//         println!("Error sending message: {why:?}")
//     }
// }

pub async fn raid_progress_message() -> Result<String, reqwest_error> {
    let url = "https://raider.io/api/v1/guilds/profile?region=us&realm=arthas&name=mud%20hut%20gang&fields=raid_progression";

    let response: GuildProfile = reqwest::get(url).await?.json().await?;

    let mut progress_message = format!("Raid Progress for {}:\n", response.name);

    for (raid_name, progression) in response.raid_progression {
        progress_message.push_str(&format!(
            "{}: {}\n Normal: {}/{}\n Heroic: {}/{}\n Mythic: {}/{}\n",
            to_title_case(&raid_name.replace("-", " ")),
            progression.summary,
            progression.normal_bosses_killed,
            progression.total_bosses,
            progression.heroic_bosses_killed,
            progression.total_bosses,
            progression.mythic_bosses_killed,
            progression.total_bosses
        ));
    }

    Ok(progress_message)
}

fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut result = first.to_uppercase().to_string();
                    result.push_str(&chars.as_str().to_lowercase());
                    result
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
