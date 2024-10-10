use crate::models::poise_required_structs::{Context, Error as poise_Error};
use crate::models::raiderio_structs::*;
use reqwest::Error as reqwest_error;

#[poise::command(slash_command, prefix_command)]
pub async fn progression(ctx: Context<'_>) -> Result<(), poise_Error> {
    let message = raid_progress_message().await.unwrap();
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn character_progression(ctx: Context<'_>) -> Result<(), poise_Error> {
    let message = all_characters_scores_message().await.unwrap();
    ctx.say(message).await?;
    Ok(())
}

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

pub async fn all_characters_scores_message() -> Result<String, reqwest_error> {
    let members = get_characters_in_guild().await?;

    let mut message: String = format!("Scores for Mud Hut Gang");

    for member in members {
        let score = get_character_score(member.clone().character).await?;
        let member_message = &format!("{} : {}\n", member.character.name, score);
        print!("{}", member_message);
        message.push_str(member_message);
    }

    Ok(message)
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

pub async fn get_characters_in_guild() -> Result<Vec<GuildMember>, reqwest_error> {
    let url = "https://raider.io/api/v1/guilds/profile?region=us&realm=arthas&name=mud%20hut%20gang&fields=members";

    let response: GuildProfileMembers = reqwest::get(url).await?.json().await?;
    println!("got guild members: {}", response.members.len());
    Ok(response.members)
}

pub async fn get_character_score(character: Character) -> Result<f32, reqwest_error> {
    let url = format!("https://raider.io/api/v1/characters/profile?region=us&realm={}&name={}&fields=mythic_plus_scores_by_season%3Acurrent", character.realm, character.name);

    let response: CharacterProfile = reqwest::get(url).await?.json().await?;
    Ok(response
        .mythic_plus_scores_by_season
        .first()
        .unwrap()
        .scores
        .all)
}
