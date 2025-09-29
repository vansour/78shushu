use serde::{Deserialize, Serialize};

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
