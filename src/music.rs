use axum::response::Json as ResponseJson;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

// 音乐播放器相关接口
pub async fn get_playlist() -> ResponseJson<serde_json::Value> {
    let playlist = Playlist {
        songs: vec![Song {
            title: "最后一哈".to_string(),
            artist: "游戏原声".to_string(),
            duration: "3:27".to_string(),
            file: "/static/music/最后一哈.mp3".to_string(),
        }],
    };

    ResponseJson(json!(playlist))
}