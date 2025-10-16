use rusqlite::{Connection, Result, Error};
use log::{info, error};
use function_name::named;


#[named]
pub fn init_database() -> Result<Connection, Error> {
    let database = Connection::open("../data/database.db")
        .map_err(|e| {
            error!("Database couldn't be initialized: {}. At {}::{}", e, file!(), function_name!());
            e
        })?;

    let query = "
        CREATE TABLE IF NOT EXISTS artists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            image_path TEXT
        );

        CREATE TABLE IF NOT EXISTS songs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            duration INTEGER
        );
    ";  

    database.execute_batch(query).map_err(|err| {
        error!("Failed to initialise the database and to create tables: {}. At {}::{}", err, file!(), function_name!());
        err
    })?;

    info!("Tables created successfully!");
    Ok(database)
}
