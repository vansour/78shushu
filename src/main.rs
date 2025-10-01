mod error;
mod music;
mod question;
mod random;
mod routes;

use axum::http::{header, HeaderValue};
use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    set_header::SetResponseHeaderLayer,
};

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志记录
    tracing_subscriber::fmt::init();

    // 配置CORS（生产环境应该限制为特定域名）
    let cors = CorsLayer::new()
        .allow_origin(Any) // 在生产环境中应该替换为特定域名
        .allow_methods(Any)
        .allow_headers(Any);

    // 配置静态文件服务，添加缓存头
    let static_files = ServeDir::new("static")
        .precompressed_gzip()
        .precompressed_br();

    let app = Router::new()
        .route("/", get(index))
        .merge(routes::random_routes::router())
        .merge(routes::question_routes::router())
        .merge(routes::music_routes::router())
        .nest_service("/static", static_files)
        .layer(cors)
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=3600"),
        ));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 三角洲鼠鼠工具启动！访问 http://localhost:3000");
    tracing::info!("📝 新功能：三角洲高考已上线！");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("无法绑定地址 {}: {}", addr, e))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("服务启动失败: {}", e))?;

    Ok(())
}
