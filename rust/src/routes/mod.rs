use axum::{Router, routing::{get, post, put, delete}};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;


mod artist_route;
pub mod album_route;

pub type SharedDatabase = Arc<Mutex<Connection>>;

pub fn create_router(database: SharedDatabase) -> Router {
    Router::new()
        // Artists routes
        .route("/artists", get(artist_route::get_all_artists))
        .route("/artists/{id}", get(artist_route::get_artist))
        .route("/artists", post(artist_route::create_artist))
        .route("/artists/{id}", put(artist_route::update_artist))
        .route("/artists/{id}", delete(artist_route::delete_artist))
        // Albums routes
        .route("/albums", get(album_route::get_all_albums))
        .route("/albums/{id}", get(album_route::get_album))
        .route("/albums", post(album_route::create_album))
        .route("/albums/{id}", put(album_route::update_album))
        .route("/albums/{id}", delete(album_route::delete_album))
        // Artist albums nested route
        .route("/artists/{id}/albums", get(album_route::get_albums_by_artist))
        .with_state(database)
}


