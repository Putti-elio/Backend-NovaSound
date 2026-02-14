use serde::{Serialize, Deserialize};
use tokio::time;

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub duration: time,
    pub image_path: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateArtist {
    pub name: String,
    pub duration: time,
}
