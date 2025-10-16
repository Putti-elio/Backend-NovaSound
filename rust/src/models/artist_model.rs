use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
    pub image_path: String, 
}

#[derive(Debug, Deserialize)]
pub struct CreateArtist {
    pub name: String,
}
