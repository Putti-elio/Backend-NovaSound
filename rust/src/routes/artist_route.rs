use axum::{extract::{State, Json}, http::StatusCode};
use log::{info, error};
use function_name::named;

use crate::models::artist_model::{Artist, CreateArtist};
use crate::services::artist_service;
use crate::routes::SharedDatabase;

// GET 
#[named]
pub async fn get_all_artists(State(database): State<SharedDatabase>) 
    -> Result<Json<Vec<Artist>>, StatusCode> {
    
    let db = database.lock().unwrap();
    
    match artist_service::get_all_artists(&db) {
        Ok(artists) => Ok(Json(artists)),
        Err(err) => {
            error!("Database error couldn't get artists. {} At {}::{} ", err, file!(), function_name!());
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// POST
#[named]
pub async fn create_artist(State(database): State<SharedDatabase>, Json(payload): Json<CreateArtist>) 
    -> (StatusCode, &'static str) { 

    let db = database.lock().unwrap();

    info!("{}", &payload.name);
    
    match artist_service::create_artist(&db, &payload.name) {
        Ok(()) => (StatusCode::CREATED, "Artist created successfully"),
        Err(err) => {
            error!("Database error couldn't create artist. {} At {}::{} ", err, file!(), function_name!()); 
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create artist")
        },
    }
}
