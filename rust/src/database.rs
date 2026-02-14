use function_name::named;
use log::{error, info};
use rusqlite::{Connection, Error, Result};

#[named]
pub fn init_database() -> Result<Connection, Error> {
    let database = Connection::open("data/database.db").map_err(|e| {
        error!(
            "Database couldn't be initialized: {}. At {}::{}",
            e,
            file!(),
            function_name!()
        );
        e
    })?;

    let query = "
        CREATE TABLE IF NOT EXISTS artists (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            image_path TEXT
        );

        CREATE TABLE IF NOT EXISTS songs (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            duration INTEGER,
            artist_id TEXT,
            FOREIGN KEY (artist_id) REFERENCES artists(id)
        );

        CREATE TABLE IF NOT EXISTS albums (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            total_duration INTEGER DEFAULT 0,
            release_date INTEGER,
            artist_id TEXT NOT NULL,
            image_path TEXT,
            FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE
        );
    ";

    database.execute_batch(query).map_err(|err| {
        error!(
            "Failed to initialise the database and to create tables: {}. At {}::{}",
            err,
            file!(),
            function_name!()
        );
        err
    })?;

    info!("Tables created successfully!");
    Ok(database)
}
