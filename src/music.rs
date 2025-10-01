use axum::response::Json as ResponseJson;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub author: String,
    pub duration: String,
    pub file: String,
    pub bilibili_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistItem {
    pub id: String,
    pub name: String,
    pub songs: Vec<Song>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicData {
    pub playlists: Vec<PlaylistItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub songs: Vec<Song>,
}

// 用于匹配JSON文件结构的辅助结构
#[derive(Debug, Serialize, Deserialize)]
struct MusicJsonRoot {
    playlists: Vec<PlaylistWithId>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlaylistWithId {
    id: String,
    name: String,
    songs: Vec<Song>,
}

// 音乐播放器相关接口
pub async fn get_playlist() -> Result<ResponseJson<serde_json::Value>, AppError> {
    let content = std::fs::read_to_string("static/music.json")
        .map_err(|e| AppError::MusicError(format!("无法读取 music.json 文件: {}", e)))?;
    
    let music_data: MusicData = serde_json::from_str(&content)
        .map_err(|e| AppError::MusicError(format!("无法解析 music.json 文件: {}", e)))?;
    
    // 返回完整的播放列表数据
    if music_data.playlists.is_empty() {
        return Err(AppError::MusicError("music.json 中没有找到播放列表".to_string()));
    }

    Ok(ResponseJson(json!(music_data)))
}