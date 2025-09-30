use axum::{routing::get, Router};

pub mod random_routes {
    use super::*;
    use crate::random;

    pub fn router() -> Router {
        Router::new()
            .route("/api/generate/loadout", get(random::generate_full_loadout))
            .route("/api/generate/map", get(random::generate_map))
            .route("/api/generate/operator", get(random::generate_operator))
            .route("/api/generate/weapon", get(random::generate_weapon))
            .route("/api/generate/helmet", get(random::generate_helmet))
            .route("/api/generate/armor", get(random::generate_armor))
    }
}

pub mod question_routes {
    use super::*;
    use crate::question;

    pub fn router() -> Router {
        Router::new()
            .route("/api/exam/question", get(question::get_random_question))
            .route("/api/exam/question/category", get(question::get_question_by_category))
            .route("/api/exam/question/difficulty", get(question::get_question_by_difficulty))
            .route("/api/exam/answer", axum::routing::post(question::submit_answer))
    }
}

pub mod music_routes {
    use super::*;
    use crate::music;

    pub fn router() -> Router {
        Router::new()
            .route("/api/music/playlist", get(music::get_playlist))
    }
}