use std::time::{Duration, Instant};

use reqwest::Error;
use rusqlite::{params, Connection, Result as SqliteResult};
use tokio::time::sleep;

use crate::{auth::get_global_token, models::{blizzard_structs::character_professions::{ProfileResponse, Recipe}, db_structs::{Character, DbOperations}, raiderio_structs::CharacterProfile}};

use super::db_init::get_db_path;

pub async fn schedule_data_updates() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(get_db_path())?;

    loop {
        let start = Instant::now();
        let characters = match get_all_characters(&conn){
            Ok(chars) => chars,
            Err(err) => {
                eprintln!("Failed to fetch characters from database: {}", err);
                continue;
            }
        };

        for character in characters {
            match fetch_character_score_data(&character.name, &character.server).await {
                Ok(character_profile) => {
                    let score = character_profile.mythic_plus_scores_by_season.first().unwrap().scores.all;
                    if let Err(err) = update_character_score_data(&conn, score, &character) {
                        eprintln!("Failed to update data for {}: {}", character.name, err);
                    }
                }

                Err(err) => {
                    eprintln!("Failed to fetch score data for {}: {}", character.name, err);
                }
            }
            match fetch_character_professions_data(&character.name, &character.server).await {
                Ok(ProfileResponse) => {

                }

                Err(err) => {
                    eprintln!("Failed to fetch profession data for {}: {}", character.name, err);
                }
            }
        }

        let elapsed = start.elapsed();
        if elapsed < Duration::from_secs(3600) {
            sleep(Duration::from_secs(3600) - elapsed).await;
        }

    }

}

fn get_all_characters(conn: &Connection) -> SqliteResult<Vec<Character>> {
    let mut stmt = conn.prepare("SELECT guid, name, server, guild, raiderio_score, level FROM characters")?;
    let character_iter = stmt.query_map([], |row| {
        Ok(Character {
            guid: row.get(0)?,
            name: row.get(1)?,
            server: row.get(2)?,
            guild: row.get(3)?,
            raiderio_score: row.get(4)?,
            level: row.get(5)?,
        })
    })?;

    character_iter.collect()
}

async fn fetch_character_score_data(name: &str, server: &str) -> Result<CharacterProfile, Error> {
    let url = format!(
        "https://raider.io/api/v1/characters/profile?region=us&realm={}&name={}&fields=mythic_plus_scores_by_season:current",
        server, name
    );
    let response = reqwest::get(&url).await?.json::<CharacterProfile>().await?;
    Ok(response)
}

async fn fetch_character_professions_data(name: &str, server: &str) -> Result<ProfileResponse, Error> {
    let api_key = get_global_token();
    let url = format!("https://{region}.api.blizzard.com/profile/wow/character/{realm_slug}/{character_name}/professions?namespace={namespace}&locale={locale}&access_token={api_key}",
    region = "us",
    realm_slug = server,
    character_name = name,
    namespace = "profile-us",
    locale = "en_US",
    api_key = api_key);

    let response = reqwest::get(&url).await?;

    let profile: ProfileResponse = response.json().await?;

    Ok(profile)
}

fn update_character_score_data(conn: &Connection, score: f32, character: &Character) -> SqliteResult<()> {
    conn.execute(
        "UPDATE characters SET raiderio_score = ?1, level = ?2 WHERE name = ?3 AND server = ?4",
        params![score, character.level, character.name, character.server],
    )?;
    Ok(())
}

fn update_caracter_profession_data(conn: &Connection, character: &Character, professions: &ProfileResponse) -> SqliteResult<()> {
    let mut recipes: Vec<Recipe> = Vec::new();

    recipes.extend(
        professions
            .primaries
            .iter()
            .filter_map(|profession| profession.tiers.first())
            .flat_map(|tier| tier.known_recipes.clone())
    );

    character.

    Ok(())
}
