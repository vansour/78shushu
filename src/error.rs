use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("随机生成失败: {0}")]
    RandomGenerationError(String),
    
    #[error("题目查询失败: {0}")]
    QuestionError(String),
    
    #[error("音乐播放器错误: {0}")]
    MusicError(String),
    
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON序列化错误: {0}")]
    JsonError(#[from] serde_json::Error),
}

// 定义Result类型别名
pub type AppResult<T> = Result<T, AppError>;

// 实现Axum的IntoResponse，让我们的错误类型可以直接作为HTTP响应返回
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        use axum::response::Json;
        use serde_json::json;

        let (status, error_message) = match &self {
            AppError::RandomGenerationError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::QuestionError(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::MusicError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::JsonError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": true,
            "message": error_message
        }));

        (status, body).into_response()
    }
}