use std::time::{Duration, Instant};

use reqwest::Error;
use rusqlite::{params, Connection, Result as SqliteResult};
use tokio::time::sleep;
use uuid::Uuid;
use std::collections::HashMap;

use crate::{auth::get_global_token, models::{blizzard_structs::{character_professions::{ProfileResponse, Recipe as RecipeAPI}, character_score::MythicKeystoneProfileResponse, guild::GuildRosterResponse, recipe_materials::RecipeResponse}, db_structs::{Character, Material as MaterialDB, Recipe as RecipeDB}}};

use super::db_init::get_db_path;


pub async fn schedule_data_updates() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut log_entries: Vec<String> = Vec::new();
        let db_path = get_db_path();
        let conn = Connection::open(&db_path)?;
        let start = Instant::now();



        if let Err(err) = update_guild_roster_data(&conn).await {
            eprintln!("Failed to update guild roster data: {}", err);
        }

        let characters = match collect_db_characters(&conn){
            Ok(chars) => chars,
            Err(err) => {
                eprintln!("Failed to fetch characters from database: {}", err);
                continue;
            }
        };

        for character in characters {
            match fetch_character_score_data(&character.name, &character.server).await {
                Ok(keystone_profile) => {
                    let score = keystone_profile.mythic_rating.rating;
                    if let Err(err) = update_character_score_data(&conn, score, &character) {
                        eprintln!("Failed to update score data for {}: {}", character.name, err);
                    }
                }

                Err(err) => {
                    eprintln!("Failed to fetch score data for {}: {}", character.name, err);
                }
            }
            match fetch_character_professions_data(&character.name, &character.server).await {
                Ok(profile_response) => {
                    if let Err(err) = update_character_profession_data(&conn, &character, &profile_response, &mut log_entries).await {
                        eprintln!("Failed to update profession data for {}: {}", character.name, err);
                    }
                }

                Err(err) => {
                    eprintln!("Failed to fetch profession data for {}: {}", character.name, err);
                }
            }
    }

    for log_entry in log_entries {
        println!("{}", log_entry);
    }

        let elapsed = start.elapsed();
        if elapsed < Duration::from_secs(3600) {
            sleep(Duration::from_secs(3600) - elapsed).await;
        }

    }

}

fn collect_db_characters(conn: &Connection) -> SqliteResult<Vec<Character>> {
    let mut stmt = conn.prepare("SELECT guid, name, server, guild, score, level FROM characters")?;
    let character_iter = stmt.query_map([], |row| {
        Ok(Character {
            guid: row.get(0)?,
            name: row.get(1)?,
            server: row.get(2)?,
            guild: row.get(3)?,
            score: row.get(4)?,
            level: row.get(5)?,
        })
    })?;

    character_iter.collect()
}

async fn update_guild_roster_data(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = get_global_token();
    println!("{}", api_key);
    let guild = "mud-hut-gang";
    let url = format!("https://{region}.api.blizzard.com/data/wow/guild/{realm_slug}/{name_slug}/roster?namespace={namespace}&locale={locale}&access_token={api_key}",
    region = "us",
    realm_slug = "arthas",
    name_slug = guild,
    namespace = "profile-us",
    locale = "en_US",
    api_key = api_key);
    let response = reqwest::get(&url).await?;
    println!("{}", url);
    println!("{:?}", response);
    // let profile: GuildRosterResponse = response.json().await?;

    // for member in profile.members {
    //     println!("updating character: {}-{}", member.character.name, member.character.realm.slug);
    //     let mut stmt = conn.prepare("SELECT level FROM characters WHERE name = ?1 AND server =?2")?;
    //     let member_exists = stmt.exists(params![member.character.name, member.character.realm.slug])?;

    //     if !member_exists {
    //         conn.execute("INSERT INTO characters (guid, name, server, guild, level) VALUES (?, ?, ?, ?, ?)",
    //          params![generate_guid(), member.character.name, member.character.realm.slug, guild, member.character.level])?;
    //     } else {
    //         let db_level: u32 = conn.query_row("SELECT level FROM characters WHERE name = ?1 AND server = ?2", params![member.character.name, member.character.realm.slug], |row| row.get(0),)?;

    //         if db_level != member.character.level {
    //             conn.execute("UPDATE characters SET level = ?1 WHERE name = ?2 AND server =?3", params![member.character.level, member.character.name, member.character.realm.slug])?;

    //         }
    //     }
    // }

    Ok(())
}

async fn fetch_character_score_data(name: &str, server: &str) -> Result<MythicKeystoneProfileResponse, Error> {
    let api_key = get_global_token();
    let url = format!("https://{region}.api.blizzard.com/profile/wow/character/{realm_slug}/{character_name}/mythic-keystone-profile/season/{seasonId}?namespace={namespace}&locale={locale}&access_token={api_key}",
    region = "us",
    realm_slug = server.to_lowercase(),
    character_name = name.to_lowercase(),
    seasonId = 13,
    namespace = "profile-us",
    locale = "en_US",
    api_key = api_key);

    let response = reqwest::get(&url).await?;

    let profile: MythicKeystoneProfileResponse = response.json().await?;

    Ok(profile)
}

fn update_character_score_data(conn: &Connection, score: f64, character: &Character) -> SqliteResult<()> {
    conn.execute(
        "UPDATE characters SET score = ?1, level = ?2 WHERE name = ?3 AND server = ?4",
        params![score, character.level, character.name, character.server],
    )?;
    Ok(())
}

async fn fetch_character_professions_data(name: &str, server: &str) -> Result<ProfileResponse, Error> {
    let api_key = get_global_token();
    let url = format!("https://{region}.api.blizzard.com/profile/wow/character/{realm_slug}/{character_name}/professions?namespace={namespace}&locale={locale}&access_token={api_key}",
    region = "us",
    realm_slug = server.to_lowercase(),
    character_name = name.to_lowercase(),
    namespace = "profile-us",
    locale = "en_US",
    api_key = api_key);

    let response = reqwest::get(&url).await?;

    let profile: ProfileResponse = response.json().await?;

    Ok(profile)
}

async fn update_character_profession_data(conn: &Connection, character: &Character, professions: &ProfileResponse, log_entries: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking recipes for {}", character.name);
    
    for profession in &professions.primaries {
        for tier in &profession.tiers {
            if !tier.tier.name.contains("Algari") {
                println!("Skipping non-War Within tier: {}", tier.tier.name);
                continue;
            }

            println!("Processing profession tier: {}", tier.tier.name);

            let recipes_from_api: &Vec<RecipeAPI> = &tier.known_recipes;
            let profession_name = &profession.profession.name;
            let tier_name = &tier.tier.name;

            match fetch_and_sync_recipes(conn, recipes_from_api, profession_name, tier_name).await {
                Ok(recipes) => {
                    for recipe in recipes {
                        if let Err(err) = sync_recipe_to_character(conn, &recipe, character, log_entries) {
                            eprintln!("Error syncing recipe {} for {}: {}", recipe.name, character.name, err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to sync recipes for profession {}: {}", profession_name, err);
                }
            }
        }
    }

    Ok(())
}

fn sync_recipe_to_character(conn: &Connection, recipe: &RecipeDB, character: &Character, log_entries: &mut Vec<String>) -> SqliteResult<()> {
    let mut stmt = conn.prepare("SELECT character_guid, recipe_guid FROM character_recipes WHERE character_guid = ?1 AND recipe_guid = ?2")?;
    let junction_exists = stmt.exists(params![character.guid, recipe.guid])?;

    if !junction_exists {
        conn.execute("INSERT INTO character_recipes (character_guid, recipe_guid) VALUES (?1, ?2)", params![character.guid, recipe.guid])?;
        log_entries.push(format!("{}-{}", character.name, recipe.name));
    }
    Ok(())
}

async fn fetch_and_sync_recipes(conn: &Connection, recipes_from_api: &[RecipeAPI], profession: &String, teir: &String) -> Result<Vec<RecipeDB>, Box<dyn std::error::Error>> {
    let mut db_recipes: Vec<RecipeDB> = Vec::new();

    for recipe in recipes_from_api {
        let recipe_guid;
        let mut stmt = conn.prepare("SELECT guid, wow_id, name, profession FROM recipes WHERE wow_id = ?1")?;
        let recipe_exists = stmt.exists(params![recipe.id])?;

        if !recipe_exists {
            recipe_guid = generate_guid();
            conn.execute("INSERT INTO recipes (guid, wow_id, name, profession, teir) VALUES (?, ?, ?, ?, ?)", params![recipe_guid, recipe.id, recipe.name, profession, teir])?;
        } else {
            recipe_guid = conn.query_row("SELECT guid FROM recipes WHERE wow_id = ?1", params![recipe.id], |row| row.get(0))?;
        }

        db_recipes.push(RecipeDB {
            guid: recipe_guid,
            wow_id: recipe.id,
            name: recipe.name.clone(),
            profession: profession.clone()
        });

        if let Err(err) = fetch_and_sync_materials(conn, recipe.id).await {
            eprintln!("Failed to sync materials for recipe {}: {}", recipe.name, err);
        }
    }

    Ok(db_recipes)
}

async fn fetch_and_sync_materials(conn: &Connection, recipe_id: u32) -> Result<HashMap<MaterialDB, u32>, Box<dyn std::error::Error>> {
    let api_key = get_global_token();
    let url = format!("https://{region}.api.blizzard.com/data/wow/recipe/{recipeId}?namespace=static-{namespace}&locale={locale}&access_token={api_key}",
    region = "us",
    recipeId = recipe_id,
    namespace = "us",
    locale = "en_US",
    api_key = api_key);

    let response = reqwest::get(&url).await?;

    let profile: RecipeResponse = response.json().await?;


    let mut db_materials: HashMap<MaterialDB, u32> = HashMap::new();

    for reagent in profile.reagents {
        let mut stmt = conn.prepare("Select guid, wow_id, name, rank FROM materials WHERE wow_id = ?1")?;
        let material_exists = stmt.exists(params![reagent.reagent.id])?;

        if !material_exists {
            conn.execute("INSERT INTO materials (guid, wow_id, name, rank) VALUES (?, ?, ?, ?)", params![generate_guid(), reagent.reagent.id, reagent.reagent.name, 3])?;
        }

        let mut stmt = conn.prepare("SELECT guid, wow_id, name, rank FROM materials WHERE wow_id = ?1")?;
        let mut rows = stmt.query(params![reagent.reagent.id])?;

        while let Some(row) = rows.next()? {
            db_materials.insert(MaterialDB {
                guid: row.get(0)?,
                wow_id: row.get(1)?,
                name: row.get(2)?,
                rank: row.get(3)?
                },
                reagent.quantity,
            );
        }
    }


    Ok(db_materials)
}

fn generate_guid() -> String {
    let id = Uuid::now_v7();
    id.to_string()
}