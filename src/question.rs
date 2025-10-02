use crate::error::AppError;
use axum::{extract::Json, response::Json as ResponseJson};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: u32,
    pub question: String,
    pub options: [String; 4], // A, B, C, D 四个选项
    pub correct_answer: u8,   // 0-3 对应 A-D
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
}

// 题库
pub struct QuestionBank;

impl QuestionBank {
    pub fn get_all_questions() -> Result<Vec<Question>, AppError> {
        tracing::debug!(target: "question", "开始加载题库");

        // 尝试从JSON文件读取数据
        let content = std::fs::read_to_string("static/question.json").map_err(|e| {
            tracing::error!(target: "question", "读取题库配置文件失败: {}", e);
            AppError::InternalServerError(format!("无法读取题库配置文件: {}", e))
        })?;

        let questions: Vec<Question> = serde_json::from_str(&content).map_err(|e| {
            tracing::error!(target: "question", "解析题库配置文件失败: {}", e);
            AppError::InternalServerError(format!("题库配置文件格式错误: {}", e))
        })?;

        if questions.is_empty() {
            tracing::error!(target: "question", "题库为空");
            return Err(AppError::InternalServerError(
                "题库配置文件中没有找到题目".to_string(),
            ));
        }

        tracing::info!(target: "question", "成功加载题库，共 {} 道题目", questions.len());
        Ok(questions)
    }

    pub fn get_question_by_id(id: u32) -> Result<Option<Question>, AppError> {
        tracing::debug!(target: "question", "查询题目 ID: {}", id);
        let questions = Self::get_all_questions()?;
        let result = questions.into_iter().find(|q| q.id == id);

        if result.is_some() {
            tracing::info!(target: "question", "找到题目 ID: {}", id);
        } else {
            tracing::warn!(target: "question", "未找到题目 ID: {}", id);
        }

        Ok(result)
    }
}

// 题目生成器
pub struct QuestionGenerator;

impl QuestionGenerator {
    pub fn get_random_question() -> Result<QuestionResponse, AppError> {
        tracing::info!(target: "question", "生成随机题目");

        let mut rng = rand::thread_rng();
        let questions = QuestionBank::get_all_questions()?;
        let question = questions
            .choose(&mut rng)
            .ok_or_else(|| {
                tracing::error!(target: "question", "题库为空，无法生成随机题目");
                AppError::InternalServerError("题库为空".to_string())
            })?
            .clone();

        tracing::info!(target: "question", "已生成随机题目 ID: {}", question.id);
        Ok(Self::format_question_response(question))
    }

    pub fn check_answer(
        question_id: u32,
        selected_option: u8,
    ) -> Result<Option<AnswerResult>, AppError> {
        tracing::info!(target: "question", "检查答案 - 题目ID: {}, 选项: {}", question_id, selected_option);

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

            if is_correct {
                tracing::info!(target: "question", "答案正确 - 题目ID: {}", question_id);
            } else {
                tracing::info!(target: "question", "答案错误 - 题目ID: {}, 正确答案: {}", question_id, question.correct_answer);
            }

            Ok(Some(AnswerResult {
                is_correct,
                correct_answer: question.correct_answer,
                correct_option,
            }))
        } else {
            tracing::warn!(target: "question", "题目不存在 - 题目ID: {}", question_id);
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
}

// 三角洲高考相关接口
pub async fn get_random_question() -> Result<ResponseJson<serde_json::Value>, AppError> {
    tracing::info!(target: "question", "API调用: 获取随机题目");
    let question_response = QuestionGenerator::get_random_question()?;
    tracing::debug!(target: "question", "返回题目ID: {}", question_response.question.id);
    Ok(ResponseJson(json!(question_response)))
}

pub async fn submit_answer(
    Json(submission): Json<AnswerSubmission>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    tracing::info!(target: "question", "API调用: 提交答案 - 题目ID: {}, 选项: {}", 
                   submission.question_id, submission.selected_option);

    match QuestionGenerator::check_answer(submission.question_id, submission.selected_option)? {
        Some(result) => {
            tracing::info!(target: "question", "返回答案检查结果: {}", 
                          if result.is_correct { "正确" } else { "错误" });
            Ok(ResponseJson(json!(result)))
        }
        None => {
            tracing::warn!(target: "question", "题目未找到: {}", submission.question_id);
            Err(AppError::NotFound("题目未找到".to_string()))
        }
    }
}
