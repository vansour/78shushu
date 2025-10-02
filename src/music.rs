use crate::error::AppError;
use axum::response::Json as ResponseJson;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

// 音乐播放器相关接口
pub async fn get_playlist() -> Result<ResponseJson<serde_json::Value>, AppError> {
    tracing::info!(target: "music", "开始获取播放列表");

    let content = std::fs::read_to_string("static/music.json").map_err(|e| {
        tracing::error!(target: "music", "读取 music.json 失败: {}", e);
        AppError::MusicError(format!("无法读取 music.json 文件: {}", e))
    })?;

    let music_data: MusicData = serde_json::from_str(&content).map_err(|e| {
        tracing::error!(target: "music", "解析 music.json 失败: {}", e);
        AppError::MusicError(format!("无法解析 music.json 文件: {}", e))
    })?;

    // 返回完整的播放列表数据
    if music_data.playlists.is_empty() {
        tracing::warn!(target: "music", "music.json 中没有播放列表");
        return Err(AppError::MusicError(
            "music.json 中没有找到播放列表".to_string(),
        ));
    }

    tracing::info!(target: "music", "成功获取播放列表，共 {} 个列表", music_data.playlists.len());
    Ok(ResponseJson(json!(music_data)))
}
