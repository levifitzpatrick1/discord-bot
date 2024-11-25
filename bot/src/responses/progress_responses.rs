use std::env;
use tokio::task;
use crate::models::poise_required_structs::{Context, Error as poise_Error};
use crate::models::db_structs::Character;
use crate::models::raiderio_structs::{CharacterResponse, GuildProfile};
use reqwest::Error as reqwest_error;
use rusqlite::{params, Connection, OptionalExtension, Result as SqliteResult};
use uuid::Uuid;

#[poise::command(slash_command, prefix_command)]
pub async fn progression(ctx: Context<'_>) -> Result<(), poise_Error> {
    let message = raid_progress_message().await.unwrap();
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn character_progression(
    ctx: Context<'_>,
    #[description = "Character Name"] character_name: String,
    #[description = "Character Server"] character_server: String,
) -> Result<(), poise_Error> {
    let _ = ctx.defer().await;
    let proper_character_name = to_title_case(&character_name);
    let proper_character_server = character_server.to_ascii_lowercase().replace(" ", "-");

    let name = proper_character_name.clone();
    let server = proper_character_server.clone();

    let message = task::spawn_blocking(move || {
        let db_message = handle_character_db_operations(&name, &server)?;
        Ok::<String, Box<dyn std::error::Error + Send + Sync>>(db_message)
    })
    .await??;

    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn update_character(
    ctx: Context<'_>,
    #[description = "Character Name"] character_name: String,
    #[description = "Character Server"] character_server: String,
) -> Result<(), poise_Error> {
    let _ = ctx.defer().await;

    let proper_character_name = to_title_case(&character_name);
    let proper_character_server = character_server.to_ascii_lowercase().replace(" ", "-");

    let score = fetch_character_from_raider_io(&proper_character_name, &proper_character_server).await?;

    let name = proper_character_name.clone();
    let server = proper_character_server.clone();
    let message = task::spawn_blocking(move || {
        update_character_in_db(&name, &server, score)
    })
    .await??;

    ctx.say(message).await?;
    Ok(())
}

fn handle_character_db_operations(name: &str, server: &str) -> SqliteResult<String> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
    let conn = Connection::open(db_url)?;
    
    let mut stmt = conn.prepare(
        "SELECT guid, name, server, guild, score, level
        FROM characters
        WHERE name = ?1 AND server = ?2"
    )?;

    let character_iter = stmt.query_map(params![name, server], |row| {
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

    if let Some(char) = character {
        return Ok(format!(
            "Name: {}, Server: {}, Score: {}",
            char.name,
            char.server,
            char.score.unwrap_or(0.0)
        ));
    }

    Ok(format!(
        "Character not found in database. Use the update_character command to fetch from Raider.io"
    ))
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

fn update_character_in_db(name: &str, server: &str, score: f64) -> SqliteResult<String> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
    let conn = Connection::open(db_url)?;

    let mut stmt = conn.prepare(
        "SELECT guid FROM characters WHERE name = ? AND server = ?"
    )?;
    
    let existing_guid: Option<String> = stmt
        .query_row(params![name, server], |row| row.get(0))
        .optional()?;
    
    match existing_guid {
        Some(guid) => {
            conn.execute(
                "UPDATE characters SET score = ? WHERE guid = ?",
                params![score, guid]
            )?;
        },
        None => {
            let new_guid = generate_guid();
            conn.execute(
                "INSERT INTO characters (guid, name, server, score, level, guild)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![new_guid, name, server, score, 80, ""]
            )?;
        }
    }

    Ok(format!(
        "Character Updated: Name: {}, Server: {}, Score: {}",
        name, server, score
    ))
}

async fn fetch_character_from_raider_io(name: &str, server: &str) -> Result<f64, reqwest_error> {
    let url = format!(
        "https://raider.io/api/v1/characters/profile?region=us&realm={}&name={}&fields=mythic_plus_scores_by_season%3Acurrent",
        server, name
    );

    let response: CharacterResponse = reqwest::get(&url)
        .await?
        .json()
        .await?;

    if let Some(season) = response.mythic_plus_scores_by_season.first() {
        return Ok(season.scores.all);
    }

    Ok(0.0)
}

fn generate_guid() -> String {
    let id = Uuid::now_v7();
    id.to_string()
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