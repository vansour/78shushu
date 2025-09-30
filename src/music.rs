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
    // 尝试从JSON文件读取数据
    let content = std::fs::read_to_string("static/music.json")
        .map_err(|e| AppError::new(format!("无法读取音乐配置文件 static/music.json: {}", e)))?;
    
    let playlist = serde_json::from_str::<Playlist>(&content)
        .map_err(|e| AppError::new(format!("音乐配置文件格式错误: {}", e)))?;

    Ok(ResponseJson(json!(playlist)))
}