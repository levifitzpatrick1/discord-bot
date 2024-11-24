use std::env;

use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Material {
    pub guid: String,
    pub wow_id: i32,
    pub name: String,
    pub rank: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub guid: String,
    pub wow_id: u32,
    pub name: String,
    pub profession: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeMaterial {
    pub recipe_guid: String,
    pub material_guid: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub guid: String,
    pub name: String,
    pub server: String,
    pub guild: Option<String>,
    pub score: Option<f64>,
    pub level: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRecipe {
    pub character_guid: String,
    pub recipe_guid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bank {
    pub wow_id: i32,
    pub material_guid: String,
    pub count: i32,
}

pub trait DbOperations {
    fn insert(&self) -> SqliteResult<()>;
    fn update(&self) -> SqliteResult<()>;
    fn delete(&self) -> SqliteResult<()>;
}
impl DbOperations for Material {
    fn insert(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "INSERT INTO materials (guid, wow_id, name, rank) VALUES (?1, ?2, ?3, ?4)",
            params![self.guid, self.wow_id, self.name, self.rank],
        )?;
        Ok(())
    }

    fn update(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "UPDATE materials SET wow_id = ?1, name = ?2, rank = ?3 WHERE guid = ?4",
            params![self.wow_id, self.name, self.rank, self.guid],
        )?;
        Ok(())
    }

    fn delete(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute("DELETE FROM materials WHERE guid = ?1", params![self.guid])?;
        Ok(())
    }
}
impl DbOperations for Recipe {
    fn insert(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "INSERT INTO recipes (guid, wow_id, name, profession) VALUES (?1, ?2, ?3, ?4)",
            params![self.guid, self.wow_id, self.name, self.profession],
        )?;
        Ok(())
    }

    fn update(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "UPDATE recipes SET wow_id = ?1, name = ?2, profession = ?3 WHERE guid = ?4",
            params![self.wow_id, self.name, self.profession, self.guid],
        )?;
        Ok(())
    }

    fn delete(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute("DELETE FROM recipes WHERE guid = ?1", params![self.guid])?;
        Ok(())
    }
}
impl DbOperations for RecipeMaterial {
    fn insert(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "INSERT INTO recipe_materials (recipe_guid, material_guid, quantity) VALUES (?1, ?2, ?3)",
            params![self.recipe_guid, self.material_guid, self.quantity],
        )?;
        Ok(())
    }

    fn update(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "UPDATE recipe_materials SET material_guid = ?1, quantity = ?2 WHERE recipe_guid = ?3",
            params![self.material_guid, self.quantity, self.recipe_guid],
        )?;
        Ok(())
    }

    fn delete(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute("DELETE FROM recipe_materials WHERE recipe_guid = ?1 AND material_guid = ?2", 
                     params![self.recipe_guid, self.material_guid])?;
        Ok(())
    }
}
impl DbOperations for Character {
    fn insert(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "INSERT INTO characters (guid, name, server, guild, score, level) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![self.guid, self.name, self.server, self.guild, self.score, self.level],
        )?;
        Ok(())
    }

    fn update(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "UPDATE characters SET name = ?1, server = ?2, guild = ?3 score = ?4, level = ?5 WHERE guid = ?6",
            params![self.name, self.server, self.score, self.level, self.guid],
        )?;
        Ok(())
    }

    fn delete(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute("DELETE FROM characters WHERE guid = ?1", params![self.guid])?;
        Ok(())
    }
}
impl DbOperations for CharacterRecipe {
    fn insert(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "INSERT INTO character_recipes (character_guid, recipe_guid) VALUES (?1, ?2)",
            params![self.character_guid, self.recipe_guid],
        )?;
        Ok(())
    }

    fn update(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "UPDATE character_recipes SET recipe_guid = ?1 WHERE character_guid = ?2",
            params![self.recipe_guid, self.character_guid],
        )?;
        Ok(())
    }

    fn delete(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute("DELETE FROM character_recipes WHERE character_guid = ?1 AND recipe_guid = ?2", 
                     params![self.character_guid, self.recipe_guid])?;
        Ok(())
    }
}
impl DbOperations for Bank {
    fn insert(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "INSERT INTO bank (wow_id, material_guid, count) VALUES (?1, ?2, ?3)",
            params![self.wow_id, self.material_guid, self.count],
        )?;
        Ok(())
    }

    fn update(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute(
            "UPDATE bank SET material_guid = ?1, count = ?2 WHERE wow_id = ?3",
            params![self.material_guid, self.count, self.wow_id],
        )?;
        Ok(())
    }

    fn delete(&self) -> SqliteResult<()> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        conn.execute("DELETE FROM bank WHERE wow_id = ?1", params![self.wow_id])?;
        Ok(())
    }
}

impl Material {
    pub fn find_by_name(name: &str) -> SqliteResult<Vec<Material>> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare("SELECT guid, wow_id, name, rank FROM materials WHERE name LIKE ?1")?;
        let material_iter = stmt.query_map(params![format!("%{}%", name)], |row| {
            Ok(Material {
                guid: row.get(0)?,
                wow_id: row.get(1)?,
                name: row.get(2)?,
                rank: row.get(3)?,
            })
        })?;

        material_iter.collect()
    }
}
impl Recipe {
    pub fn find_by_name(name: &str) -> SqliteResult<Vec<Recipe>> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare("SELECT guid, wow_id, name, profession FROM recipes WHERE name LIKE ?1")?;
        let recipe_iter = stmt.query_map(params![format!("%{}%", name)], |row| {
            Ok(Recipe {
                guid: row.get(0)?,
                wow_id: row.get(1)?,
                name: row.get(2)?,
                profession: row.get(3)?,
            })
        })?;
    
    recipe_iter.collect()
    }

    pub fn get_materials(&self) -> SqliteResult<Vec<(Material, i32)>> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare("
            SELECT m.guid, m.wow_id, m.name, m.rank, rm.quantity
            FROM materials m
            JOIN recipe_materials rm ON m.guid = rm.material_guid
            WHERE rm.recipe_guid = ?1
        ")?;

        let material_iter = stmt.query_map(params![self.guid], |row| {
            Ok((
                Material {
                    guid: row.get(0)?,
                    wow_id: row.get(1)?,
                    name: row.get(2)?,
                    rank: row.get(3)?,
                },
                row.get(4)?,
            ))
        })?;

        material_iter.collect()
    }
}
impl Character {
    pub fn get_recipes(&self) -> SqliteResult<Vec<Recipe>> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/bot_data.db".to_string());
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare("
            SELECT r.guid, r.wow_id, r.name, r.profession
            FROM recipes r
            JOIN character_recipes cr ON r.guid = cr.recipe_guid
            WHERE cr.character_guid = ?1
        ")?;

        let recipe_iter = stmt.query_map(params![self.guid], |row| {
            Ok(Recipe {
                guid: row.get(0)?,
                wow_id: row.get(1)?,
                name: row.get(2)?,
                profession: row.get(3)?,
            })
        })?;

        recipe_iter.collect()
    }
}