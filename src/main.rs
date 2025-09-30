mod generator;
mod models;
mod question_bank;

use axum::{
    extract::{Json, Query},
    response::{Html, Json as ResponseJson},
    routing::{get, post},
    Router,
};
use generator::{QuestionGenerator, RandomGenerator};
use models::*;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tower_http::cors::Any;

#[derive(Deserialize)]
struct CategoryQuery {
    category: Option<String>,
}

#[derive(Deserialize)]
struct DifficultyQuery {
    difficulty: Option<String>,
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

// 健康检查端点
async fn health_check() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "status": "ok",
        "message": "三角洲鼠鼠工具运行正常",
        "service": "shushu78"
    }))
}

// 随机装备相关接口
async fn generate_full_loadout() -> ResponseJson<serde_json::Value> {
    let loadout = RandomGenerator::generate_loadout();
    ResponseJson(json!(loadout))
}

async fn generate_map() -> ResponseJson<serde_json::Value> {
    let map = RandomGenerator::generate_map();
    ResponseJson(json!(map))
}

async fn generate_operator() -> ResponseJson<serde_json::Value> {
    let operator = RandomGenerator::generate_operator();
    ResponseJson(json!(operator))
}

async fn generate_weapon() -> ResponseJson<serde_json::Value> {
    let weapon = RandomGenerator::generate_weapon();
    ResponseJson(json!(weapon))
}

async fn generate_helmet() -> ResponseJson<serde_json::Value> {
    let helmet = RandomGenerator::generate_helmet();
    ResponseJson(json!(helmet))
}

async fn generate_armor() -> ResponseJson<serde_json::Value> {
    let armor = RandomGenerator::generate_armor();
    ResponseJson(json!(armor))
}

// 三角洲高考相关接口
async fn get_random_question() -> ResponseJson<serde_json::Value> {
    let question_response = QuestionGenerator::get_random_question();
    ResponseJson(json!(question_response))
}

async fn get_question_by_category(
    Query(params): Query<CategoryQuery>,
) -> ResponseJson<serde_json::Value> {
    if let Some(category_str) = params.category {
        let category = match category_str.as_str() {
            "weapons" | "武器知识" => QuestionCategory::Weapons,
            "maps" | "地图熟悉" => QuestionCategory::Maps,
            "tactics" | "战术策略" => QuestionCategory::Tactics,
            "equipment" | "装备配件" => QuestionCategory::Equipment,
            "game" | "游戏机制" => QuestionCategory::GameMechanics,
            _ => return ResponseJson(json!({"error": "Invalid category"})),
        };

        if let Some(question_response) =
            QuestionGenerator::get_random_question_by_category(category)
        {
            ResponseJson(json!(question_response))
        } else {
            ResponseJson(json!({"error": "No questions found for this category"}))
        }
    } else {
        let question_response = QuestionGenerator::get_random_question();
        ResponseJson(json!(question_response))
    }
}

async fn get_question_by_difficulty(
    Query(params): Query<DifficultyQuery>,
) -> ResponseJson<serde_json::Value> {
    if let Some(difficulty_str) = params.difficulty {
        let difficulty = match difficulty_str.as_str() {
            "easy" | "简单" => Difficulty::Easy,
            "medium" | "中等" => Difficulty::Medium,
            "hard" | "困难" => Difficulty::Hard,
            _ => return ResponseJson(json!({"error": "Invalid difficulty"})),
        };

        if let Some(question_response) =
            QuestionGenerator::get_random_question_by_difficulty(difficulty)
        {
            ResponseJson(json!(question_response))
        } else {
            ResponseJson(json!({"error": "No questions found for this difficulty"}))
        }
    } else {
        let question_response = QuestionGenerator::get_random_question();
        ResponseJson(json!(question_response))
    }
}

async fn submit_answer(
    Json(submission): Json<AnswerSubmission>,
) -> ResponseJson<serde_json::Value> {
    if let Some(result) =
        QuestionGenerator::check_answer(submission.question_id, submission.selected_option)
    {
        ResponseJson(json!(result))
    } else {
        ResponseJson(json!({"error": "Question not found"}))
    }
}

// 音乐播放器相关接口
async fn get_playlist() -> ResponseJson<serde_json::Value> {
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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        // 随机装备API
        .route("/api/generate/loadout", get(generate_full_loadout))
        .route("/api/generate/map", get(generate_map))
        .route("/api/generate/operator", get(generate_operator))
        .route("/api/generate/weapon", get(generate_weapon))
        .route("/api/generate/helmet", get(generate_helmet))
        .route("/api/generate/armor", get(generate_armor))
        // 三角洲高考API
        .route("/api/exam/question", get(get_random_question))
        .route("/api/exam/question/category", get(get_question_by_category))
        .route(
            "/api/exam/question/difficulty",
            get(get_question_by_difficulty),
        )
        .route("/api/exam/answer", post(submit_answer))
        // 音乐播放器API
        .route("/api/music/playlist", get(get_playlist))
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
                .expose_headers(Any)
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 三角洲鼠鼠工具启动！访问 http://localhost:3000");
    println!("📝 新功能：三角洲高考已上线！");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
