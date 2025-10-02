use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
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

    #[error("请求参数错误: {0}")]
    BadRequest(String),

    #[error("资源未找到: {0}")]
    NotFound(String),

    #[error("内部服务器错误: {0}")]
    InternalServerError(String),
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
            AppError::RandomGenerationError(_) => {
                tracing::error!(target: "error", "随机生成错误: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::QuestionError(_) => {
                tracing::error!(target: "error", "题目查询错误: {}", self);
                (StatusCode::NOT_FOUND, self.to_string())
            }
            AppError::MusicError(_) => {
                tracing::error!(target: "error", "音乐播放器错误: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::IoError(_) => {
                tracing::error!(target: "error", "IO错误: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::JsonError(_) => {
                tracing::error!(target: "error", "JSON序列化错误: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::BadRequest(_) => {
                tracing::warn!(target: "error", "请求参数错误: {}", self);
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            AppError::NotFound(_) => {
                tracing::warn!(target: "error", "资源未找到: {}", self);
                (StatusCode::NOT_FOUND, self.to_string())
            }
            AppError::InternalServerError(_) => {
                tracing::error!(target: "error", "内部服务器错误: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = Json(json!({
            "error": true,
            "message": error_message
        }));

        (status, body).into_response()
    }
}
