mod music;
mod random;
mod question;

use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir};

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        // 随机装备API
        .route("/api/generate/loadout", get(random::generate_full_loadout))
        .route("/api/generate/map", get(random::generate_map))
        .route("/api/generate/operator", get(random::generate_operator))
        .route("/api/generate/weapon", get(random::generate_weapon))
        .route("/api/generate/helmet", get(random::generate_helmet))
        .route("/api/generate/armor", get(random::generate_armor))
        // 三角洲高考API
        .route("/api/exam/question", get(question::get_random_question))
        .route("/api/exam/question/category", get(question::get_question_by_category))
        .route("/api/exam/question/difficulty", get(question::get_question_by_difficulty))
        .route("/api/exam/answer", axum::routing::post(question::submit_answer))
        // 音乐播放器API
        .route("/api/music/playlist", get(music::get_playlist))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 三角洲鼠鼠工具启动！访问 http://localhost:3000");
    println!("📝 新功能：三角洲高考已上线！");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
