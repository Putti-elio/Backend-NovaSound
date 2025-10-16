use axum::{Router, routing::{get, post}};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;


mod artist_route;

pub type SharedDatabase = Arc<Mutex<Connection>>;

pub fn create_router(database: SharedDatabase) -> Router {
    Router::new()
        .route("/artists", get(artist_route::get_all_artists))
        .route("/artists", post(artist_route::create_artist))
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(database)
}
