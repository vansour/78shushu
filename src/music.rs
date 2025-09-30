use axum::response::Json as ResponseJson;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::error::AppError;

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
pub async fn get_playlist() -> Result<ResponseJson<serde_json::Value>, AppError> {
    // 尝试从JSON文件读取数据，如果失败则使用默认数据
    let playlist = if let Ok(content) = std::fs::read_to_string("static/music.json") {
        if let Ok(playlist) = serde_json::from_str::<Playlist>(&content) {
            playlist
        } else {
            // 默认数据作为fallback
            Playlist {
                songs: vec![Song {
                    title: "最后一哈".to_string(),
                    artist: "游戏原声".to_string(),
                    duration: "3:27".to_string(),
                    file: "/static/music/最后一哈.mp3".to_string(),
                }],
            }
        }
    } else {
        // 默认数据作为fallback
        Playlist {
            songs: vec![Song {
                title: "最后一哈".to_string(),
                artist: "游戏原声".to_string(),
                duration: "3:27".to_string(),
                file: "/static/music/最后一哈.mp3".to_string(),
            }],
        }
    };

    Ok(ResponseJson(json!(playlist)))
}