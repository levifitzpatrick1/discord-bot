use crate::db::db_init::get_db_path;
use crate::models::poise_required_structs::{Context, Error as poise_Error};
use crate::models::db_structs::Character;
use crate::models::raiderio_structs::GuildProfile;
use reqwest::Error as reqwest_error;
use rusqlite::{params, Connection, Result as SqliteResult};

#[poise::command(slash_command, prefix_command)]
pub async fn progression(ctx: Context<'_>) -> Result<(), poise_Error> {
    let message = raid_progress_message().await.unwrap();
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn character_progression(ctx: Context<'_>,
#[description = "Character Name"] character_name: String) -> Result<(), poise_Error> {
    let _ = ctx.defer().await;
    let message = character_score_message(character_name).unwrap();
    ctx.say(message).await?;
    Ok(())
}

async fn raid_progress_message() -> Result<String, reqwest_error> {
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

fn character_score_message(name: String) -> SqliteResult<String> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT guid, name, server, guild, score, level
        FROM characters
        WHERE name = ?1"
    )?;

    let character_iter = stmt.query_map(params![name], |row| {
        Ok(Character {
            guid: row.get(0)?,
            name: row.get(1)?,
            server: row.get(2)?,
            guild: row.get(3)?,
            score: row.get(4)?,
            level: row.get(5)?
        })
    })?;

    let characters = character_iter.collect::<Result<Vec<_>, _>>()?;
    let character = characters.first();

    let message = match character {
        Some(char) => format!(
            "Name: {}, Server: {}, Score: {}",
            char.name,
            char.server,
            char.score.unwrap_or(0.0)
        ),
        None => format!("No character found with the name {}", name),
    };

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

