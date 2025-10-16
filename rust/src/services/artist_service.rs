use rusqlite::{Connection, params, Error as RusqliteError};
use anyhow::{Result, Context};
use function_name::named;

use crate::models::artist_model::Artist;
use crate::utils::{log_and_context_error};
use crate::create_error;

#[named]
pub fn get_all_artists(database: &Connection) -> Result<Vec<Artist>> {
    let mut statement = database.prepare("SELECT name, image_path FROM artists")
        .map_err(|err| log_and_context_error(err, "Failed to prepare the artist query", file!(), function_name!()))?;

    let result_statement = statement.query_map([], |row| {
        Ok(Artist {
            name: row.get(0)?,
            image_path: row.get(1)?,
        })
    })
    .map_err(|err| log_and_context_error(err, "Failed to execute the artist query", file!(), function_name!()))?;

    let artists = result_statement.collect::<Result<Vec<Artist>, RusqliteError>>()
        .map_err(|err| log_and_context_error(err, "Failed to collect artists from iterator", file!(), function_name!()))?;

    Ok(artists)
}

#[named]
pub fn create_artist(database: &Connection, name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return create_error!("Artist name cannot be empty.");
    }

    let mut stmt = database.prepare("SELECT 1 FROM artists WHERE name = ?1")
    .map_err(|err| log_and_context_error(err, "Failed to prepare statement to check for existing artist.", file!(), function_name!()))?;
    
    let exists: bool = stmt.exists(params![name])
    .map_err(|err| log_and_context_error(err, "Failed to check if artist exists.", file!(), function_name!()))?;

    if exists {
        return create_error!("Artist '{}' already exists.", name);
    }


    let image_path = "/images/".to_owned()+name;
    database.execute(
        "INSERT INTO artists (name, image_path) VALUES (?1, ?2)",
        params![name, image_path],
    ).with_context(|| format!("Failed to insert artist '{}'", name))?;

    Ok(())
}
