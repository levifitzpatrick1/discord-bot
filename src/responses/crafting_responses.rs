
use rusqlite::{params, Connection, Result as SqliteResult};
use crate::models::poise_required_structs::{Context, Error as poise_Error};
use crate::db::db_init::get_db_path;
use crate::models::db_structs::Character;
use crate::models::db_structs::Material;
use crate::models::db_structs::Recipe;

#[poise::command(slash_command)]
pub async fn craft_request(
    ctx: Context<'_>,
    #[description = "Name of Item to craft"] item_name: String,
) -> Result<(), poise_Error> {
    let _ = ctx.defer();
    let recipes = Recipe::find_by_name(&item_name)?;
    
    if recipes.is_empty() {
        ctx.say(format!("Item {} not found in the recipe database.", item_name)).await?;
        return Ok(());
    }

    let recipe = &recipes[0];
    let crafters = get_crafters(&recipe)?;

    let materials = recipe.get_materials()?;

    let response = format_crafting_response(recipe, &crafters, &materials);

    ctx.say(response).await?;

    Ok(())
}

fn format_crafting_response(recipe: &Recipe, crafters: &[Character], materials: &[(Material, i32)]) -> String {
    let mut response = format!("Crafting information for {}:\n", recipe.name);
    response.push_str(&format!("Profession: {}\n\n", recipe.profession));

    if crafters.is_empty() {
        response.push_str("No one in the guild can craft this item.\n");
    } else {
        response.push_str("Can be crafted by:\n");
        for crafter in crafters {
            response.push_str(&format!("- {} ({})\n", crafter.name, crafter.server));
        }
    }

    response.push_str("\nMaterials needed:\n");

    for material in materials {
        response.push_str(&format!("- {} (Rank {}): {} needed", material.0.name, material.0.rank, material.1));
    }

    response
}

fn get_crafters(recipe: &Recipe) -> SqliteResult<Vec<Character>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare("
    SELECT c.guid, c.name, c.server, c.guild, c.score, c.level
    FROM characters c
    JOIN character_recipes cr ON c.guid = cr.character_guid
    WHERE cr.recipe_guid = ?1
    ")?;

    let character_iter = stmt.query_map(params![recipe.guid], |row| {
        Ok(Character {
            guid: row.get(0)?,
            name: row.get(1)?,
            server: row.get(2)?,
            guild: row.get(3)?,
            score: row.get(4)?,
            level: row.get(5)?
        })
    })?;

    character_iter.collect()

}