use std::{env, fs, path::PathBuf};
use rusqlite::{Connection, Result};

pub fn init_db() -> Result<()> {
    let db_path = get_db_path();
    println!("db hosted at {:?}", db_path);

    if let Some(parent) = db_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let conn = Connection::open(&db_path)?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    println!("Creating tables");
    if let Err(err) = create_tables(&conn) {
        eprintln!("Error creating tables at {:?}: {}", db_path, err)
    }
    println!("Done creating tables");

    Ok(())
}

pub fn get_db_path() -> PathBuf {
    env::var("DB_PATH")
    .map(PathBuf::from)
    .unwrap_or_else(|_| PathBuf::from("/app/data/bot_data.db"))
}


fn create_tables(conn: &Connection) -> Result<()> {
    //materials table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS materials (
            guid TEXT PRIMARY KEY,
            wow_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            rank INTEGER NOT NULL
        )",
        [],
    )?;

    //recipes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipes (
            guid TEXT PRIMARY KEY,
            wow_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            profession TEXT NOT NULL,
            teir TEXT NOT NULL
        )",
        [],
    )?;

    //recipe_materials junction table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipe_materials (
            recipe_guid TEXT NOT NULL,
            material_guid TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            PRIMARY KEY (recipe_guid, material_guid),
            FOREIGN KEY (recipe_guid) REFERENCES recipes(guid),
            FOREIGN KEY (material_guid) REFERENCES materials(guid)
        )",
        [],
    )?;

    //characters table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS characters (
            guid TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            server TEXT NOT NULL,
            guild TEXT,
            score REAL,
            level INTEGER NOT NULL
        )",
        [],
    )?;

    //character_recipes junction table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character_recipes (
            character_guid TEXT NOT NULL,
            recipe_guid TEXT NOT NULL,
            PRIMARY KEY (character_guid, recipe_guid),
            FOREIGN KEY (character_guid) REFERENCES characters(guid),
            FOREIGN KEY (recipe_guid) REFERENCES recipes(guid)
        )",
        [],
    )?;

    //bank table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bank (
            wow_id INTEGER NOT NULL,
            material_guid TEXT NOT NULL,
            count INTEGER NOT NULL,
            PRIMARY KEY (wow_id, material_guid),
            FOREIGN KEY (material_guid) REFERENCES materials(guid)
        )",
        [],
    )?;

    Ok(())
}
