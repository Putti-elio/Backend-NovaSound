use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let db = Connection::open("app.db")?;
    println!("✅ Connected to SQLite database.");
    
    let query = "
        CREATE TABLE IF NOT EXISTS artists (
            name TEXT, 
        );

        CREATE TABLE IF NOT EXISTS songs (
            name TEXT,
            duration TIME
        );
    ";  

    db.execute_batch(query){
        Ok(db) =>{
            println!("Connected to SQLite database and tables create.");
        }
        Err(e) =>{
            println!("Not connected to SQLite database.");
        }
    };
}
