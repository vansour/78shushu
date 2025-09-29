use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub songs: Vec<Song>,
}
