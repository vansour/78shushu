use axum::{
    extract::{Json, Query},
    response::Json as ResponseJson,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: u32,
    pub question: String,
    pub options: [String; 4], // A, B, C, D 四个选项
    pub correct_answer: u8,   // 0-3 对应 A-D
    pub category: QuestionCategory,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionCategory {
    #[serde(rename = "武器知识")]
    Weapons,
    #[serde(rename = "地图熟悉")]
    Maps,
    #[serde(rename = "战术策略")]
    Tactics,
    #[serde(rename = "装备配件")]
    Equipment,
    #[serde(rename = "游戏机制")]
    GameMechanics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "简单")]
    Easy,
    #[serde(rename = "中等")]
    Medium,
    #[serde(rename = "困难")]
    Hard,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionResponse {
    pub question: Question,
    pub options_labeled: Vec<String>, // 带有A、B、C、D标签的选项
}

#[derive(Debug, Deserialize)]
pub struct AnswerSubmission {
    pub question_id: u32,
    pub selected_option: u8, // 0-3 对应 A-D
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerResult {
    pub is_correct: bool,
    pub correct_answer: u8,
    pub correct_option: String,
    pub explanation: Option<String>,
}

#[derive(Deserialize)]
pub struct CategoryQuery {
    pub category: Option<String>,
}

#[derive(Deserialize)]
pub struct DifficultyQuery {
    pub difficulty: Option<String>,
}

// 题库
pub struct QuestionBank;

impl QuestionBank {
    pub fn get_all_questions() -> Result<Vec<Question>, AppError> {
        // 尝试从JSON文件读取数据
        let content = std::fs::read_to_string("static/question.json")
            .map_err(|e| AppError::InternalServerError(format!("无法读取题库配置文件: {}", e)))?;
        
        let questions: Vec<Question> = serde_json::from_str(&content)
            .map_err(|e| AppError::InternalServerError(format!("题库配置文件格式错误: {}", e)))?;
        
        if questions.is_empty() {
            return Err(AppError::InternalServerError("题库配置文件中没有找到题目".to_string()));
        }
        
        Ok(questions)
    }

    pub fn get_question_by_id(id: u32) -> Result<Option<Question>, AppError> {
        let questions = Self::get_all_questions()?;
        Ok(questions.into_iter().find(|q| q.id == id))
    }

    pub fn get_questions_by_category(category: QuestionCategory) -> Result<Vec<Question>, AppError> {
        let questions = Self::get_all_questions()?;
        Ok(questions
            .into_iter()
            .filter(|q| {
                matches!(
                    (&q.category, &category),
                    (QuestionCategory::Weapons, QuestionCategory::Weapons)
                        | (QuestionCategory::Maps, QuestionCategory::Maps)
                        | (QuestionCategory::Tactics, QuestionCategory::Tactics)
                        | (QuestionCategory::Equipment, QuestionCategory::Equipment)
                        | (
                            QuestionCategory::GameMechanics,
                            QuestionCategory::GameMechanics
                        )
                )
            })
            .collect())
    }

    pub fn get_questions_by_difficulty(difficulty: Difficulty) -> Result<Vec<Question>, AppError> {
        let questions = Self::get_all_questions()?;
        Ok(questions
            .into_iter()
            .filter(|q| {
                matches!(
                    (&q.difficulty, &difficulty),
                    (Difficulty::Easy, Difficulty::Easy)
                        | (Difficulty::Medium, Difficulty::Medium)
                        | (Difficulty::Hard, Difficulty::Hard)
                )
            })
            .collect())
    }
}

// 题目生成器
pub struct QuestionGenerator;

impl QuestionGenerator {
    pub fn get_random_question() -> Result<QuestionResponse, AppError> {
        let mut rng = rand::thread_rng();
        let questions = QuestionBank::get_all_questions()?;
        let question = questions.choose(&mut rng)
            .ok_or_else(|| AppError::InternalServerError("题库为空".to_string()))?
            .clone();

        Ok(Self::format_question_response(question))
    }

    pub fn get_random_question_by_category(category: QuestionCategory) -> Result<Option<QuestionResponse>, AppError> {
        let mut rng = rand::thread_rng();
        let questions = QuestionBank::get_questions_by_category(category)?;

        if questions.is_empty() {
            return Ok(None);
        }

        let question = questions.choose(&mut rng).unwrap().clone();
        Ok(Some(Self::format_question_response(question)))
    }

    pub fn get_random_question_by_difficulty(difficulty: Difficulty) -> Result<Option<QuestionResponse>, AppError> {
        let mut rng = rand::thread_rng();
        let questions = QuestionBank::get_questions_by_difficulty(difficulty)?;

        if questions.is_empty() {
            return Ok(None);
        }

        let question = questions.choose(&mut rng).unwrap().clone();
        Ok(Some(Self::format_question_response(question)))
    }

    pub fn check_answer(question_id: u32, selected_option: u8) -> Result<Option<AnswerResult>, AppError> {
        let question = QuestionBank::get_question_by_id(question_id)?;

        if let Some(question) = question {
            let is_correct = question.correct_answer == selected_option;
            let correct_option = format!(
                "{}. {}",
                match question.correct_answer {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    _ => "D",
                },
                question.options[question.correct_answer as usize]
            );

            Ok(Some(AnswerResult {
                is_correct,
                correct_answer: question.correct_answer,
                correct_option,
                explanation: Self::get_explanation(&question),
            }))
        } else {
            Ok(None)
        }
    }

    fn format_question_response(question: Question) -> QuestionResponse {
        let options_labeled = question
            .options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let label = match i {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    _ => "D",
                };
                format!("{}. {}", label, option)
            })
            .collect();

        QuestionResponse {
            question,
            options_labeled,
        }
    }

    fn get_explanation(question: &Question) -> Option<String> {
        match question.id {
            1 => Some(
                "AK-74是经典的苏式突击步枪，标准弹匣容量为30发，这是大多数突击步枪的标准配置。"
                    .to_string(),
            ),
            2 => {
                Some("AWP是著名的狙击步枪，具有极高的伤害和射程，适合远距离精确射击。".to_string())
            }
            3 => {
                Some("Vector冲锋枪以其超高的射速而闻名，在近距离战斗中具有压倒性优势。".to_string())
            }
            4 => Some(
                "狙击手需要占据制高点来获得更好的视野和射击角度，瞭望台是理想的狙击位置。"
                    .to_string(),
            ),
            7 => Some("医疗兵是团队的生命保障，应该优先救治受伤队友，保持团队战斗力。".to_string()),
            8 => Some("烟雾弹可以有效阻挡狙击手视线，为团队提供移动掩护。".to_string()),
            12 => {
                Some("支援兵负责为团队提供持续的火力支援和弹药补给，是团队的火力核心。".to_string())
            }
            _ => None,
        }
    }
}

// 三角洲高考相关接口
pub async fn get_random_question() -> Result<ResponseJson<serde_json::Value>, AppError> {
    let question_response = QuestionGenerator::get_random_question()?;
    Ok(ResponseJson(json!(question_response)))
}

pub async fn get_question_by_category(
    Query(params): Query<CategoryQuery>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    if let Some(category_str) = params.category {
        let category = match category_str.as_str() {
            "weapons" | "武器知识" => QuestionCategory::Weapons,
            "maps" | "地图熟悉" => QuestionCategory::Maps,
            "tactics" | "战术策略" => QuestionCategory::Tactics,
            "equipment" | "装备配件" => QuestionCategory::Equipment,
            "game" | "游戏机制" => QuestionCategory::GameMechanics,
            _ => return Err(AppError::BadRequest("无效的题目类别".to_string())),
        };

        match QuestionGenerator::get_random_question_by_category(category)? {
            Some(question_response) => Ok(ResponseJson(json!(question_response))),
            None => Err(AppError::NotFound("该类别下没有找到题目".to_string())),
        }
    } else {
        let question_response = QuestionGenerator::get_random_question()?;
        Ok(ResponseJson(json!(question_response)))
    }
}

pub async fn get_question_by_difficulty(
    Query(params): Query<DifficultyQuery>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    if let Some(difficulty_str) = params.difficulty {
        let difficulty = match difficulty_str.as_str() {
            "easy" | "简单" => Difficulty::Easy,
            "medium" | "中等" => Difficulty::Medium,
            "hard" | "困难" => Difficulty::Hard,
            _ => return Err(AppError::BadRequest("无效的难度等级".to_string())),
        };

        match QuestionGenerator::get_random_question_by_difficulty(difficulty)? {
            Some(question_response) => Ok(ResponseJson(json!(question_response))),
            None => Err(AppError::NotFound("该难度下没有找到题目".to_string())),
        }
    } else {
        let question_response = QuestionGenerator::get_random_question()?;
        Ok(ResponseJson(json!(question_response)))
    }
}

pub async fn submit_answer(
    Json(submission): Json<AnswerSubmission>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    match QuestionGenerator::check_answer(submission.question_id, submission.selected_option)? {
        Some(result) => Ok(ResponseJson(json!(result))),
        None => Err(AppError::NotFound("题目未找到".to_string())),
    }
}