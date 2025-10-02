use axum::{routing::get, Router};

pub mod random_routes {
    use super::*;
    use crate::random;

    pub fn router() -> Router {
        Router::new().route("/api/generate/loadout", get(random::generate_full_loadout))
    }
}

pub mod question_routes {
    use super::*;
    use crate::question;

    pub fn router() -> Router {
        Router::new()
            .route("/api/exam/question", get(question::get_random_question))
            .route(
                "/api/exam/answer",
                axum::routing::post(question::submit_answer),
            )
    }
}

pub mod music_routes {
    use super::*;
    use crate::music;

    pub fn router() -> Router {
        Router::new().route("/api/music/playlist", get(music::get_playlist))
    }
}
