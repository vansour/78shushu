use axum::{
    extract::{Json, Query},
    response::Json as ResponseJson,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    pub fn get_all_questions() -> Vec<Question> {
        vec![
            // 武器知识类题目
            Question {
                id: 1,
                question: "在《三角洲行动》中，AK-74突击步枪的标准弹匣容量是多少发？".to_string(),
                options: [
                    "20发".to_string(),
                    "30发".to_string(),
                    "40发".to_string(),
                    "60发".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Weapons,
                difficulty: Difficulty::Easy,
            },
            Question {
                id: 2,
                question: "以下哪种武器属于狙击步枪类别？".to_string(),
                options: [
                    "M4A1".to_string(),
                    "MP5".to_string(),
                    "AWP".to_string(),
                    "M249".to_string(),
                ],
                correct_answer: 2,
                category: QuestionCategory::Weapons,
                difficulty: Difficulty::Easy,
            },
            Question {
                id: 3,
                question: "Vector冲锋枪的主要特点是什么？".to_string(),
                options: [
                    "射程远".to_string(),
                    "射速快".to_string(),
                    "伤害高".to_string(),
                    "容量大".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Weapons,
                difficulty: Difficulty::Medium,
            },
            // 地图知识类题目
            Question {
                id: 4,
                question: "在海岸线地图中，最适合狙击手位置的是哪个区域？".to_string(),
                options: [
                    "海滩区域".to_string(),
                    "高地瞭望台".to_string(),
                    "船坞码头".to_string(),
                    "沙滩掩体".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Maps,
                difficulty: Difficulty::Medium,
            },
            Question {
                id: 5,
                question: "农场地图的主要地形特征是什么？".to_string(),
                options: [
                    "城市建筑群".to_string(),
                    "开阔田野和农舍".to_string(),
                    "工业厂房".to_string(),
                    "港口码头".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Maps,
                difficulty: Difficulty::Easy,
            },
            Question {
                id: 6,
                question: "在总部地图中，控制哪个位置最为重要？".to_string(),
                options: [
                    "外围停车场".to_string(),
                    "后勤补给区".to_string(),
                    "中央指挥室".to_string(),
                    "警卫室".to_string(),
                ],
                correct_answer: 2,
                category: QuestionCategory::Maps,
                difficulty: Difficulty::Hard,
            },
            // 战术策略类题目
            Question {
                id: 7,
                question: "在团队作战中，医疗兵应该优先做什么？".to_string(),
                options: [
                    "冲锋陷阵".to_string(),
                    "支援队友治疗".to_string(),
                    "远程狙击".to_string(),
                    "爆破任务".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Tactics,
                difficulty: Difficulty::Easy,
            },
            Question {
                id: 8,
                question: "面对敌方狙击手威胁时，最佳应对策略是？".to_string(),
                options: [
                    "正面强攻".to_string(),
                    "利用烟雾弹掩护".to_string(),
                    "原地防守".to_string(),
                    "分散逃跑".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Tactics,
                difficulty: Difficulty::Medium,
            },
            Question {
                id: 9,
                question: "在城市战中，工程师的主要作用是？".to_string(),
                options: [
                    "提供医疗支援".to_string(),
                    "布置爆炸物和修理设备".to_string(),
                    "远程火力支援".to_string(),
                    "侦察敌情".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Tactics,
                difficulty: Difficulty::Medium,
            },
            // 装备配件类题目
            Question {
                id: 10,
                question: "重型护甲相比轻型护甲的主要优势是？".to_string(),
                options: [
                    "移动速度更快".to_string(),
                    "防护能力更强".to_string(),
                    "更加轻便".to_string(),
                    "隐蔽性更好".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Equipment,
                difficulty: Difficulty::Easy,
            },
            Question {
                id: 11,
                question: "战术头盔主要提供什么保护？".to_string(),
                options: [
                    "面部保护".to_string(),
                    "头部防弹保护".to_string(),
                    "夜视功能".to_string(),
                    "通讯增强".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Equipment,
                difficulty: Difficulty::Easy,
            },
            // 游戏机制类题目
            Question {
                id: 12,
                question: "在《三角洲行动》中，支援兵的主要职责是？".to_string(),
                options: [
                    "突破敌方防线".to_string(),
                    "提供弹药和火力支援".to_string(),
                    "医疗救护".to_string(),
                    "侦察敌情".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::GameMechanics,
                difficulty: Difficulty::Easy,
            },
            Question {
                id: 13,
                question: "狙击手在队伍中的最佳定位是？".to_string(),
                options: [
                    "前线冲锋".to_string(),
                    "中距离支援".to_string(),
                    "后方远程覆盖".to_string(),
                    "近身格斗".to_string(),
                ],
                correct_answer: 2,
                category: QuestionCategory::GameMechanics,
                difficulty: Difficulty::Medium,
            },
            Question {
                id: 14,
                question: "在多人对战中，团队配合最重要的是什么？".to_string(),
                options: [
                    "个人击杀数".to_string(),
                    "沟通协调".to_string(),
                    "武器威力".to_string(),
                    "移动速度".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::GameMechanics,
                difficulty: Difficulty::Medium,
            },
            Question {
                id: 15,
                question: "突击兵在战斗中应该承担什么角色？".to_string(),
                options: [
                    "后勤支援".to_string(),
                    "主攻手突破".to_string(),
                    "医疗救护".to_string(),
                    "远程狙击".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::GameMechanics,
                difficulty: Difficulty::Easy,
            },
            // 高难度综合题目
            Question {
                id: 16,
                question: "在港口地图的夜间作战中，使用什么战术组合最有效？".to_string(),
                options: [
                    "全员使用重型武器正面强攻".to_string(),
                    "狙击手控制制高点，突击兵从侧翼包抄".to_string(),
                    "所有人集中一点突破".to_string(),
                    "分散行动各自为战".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Tactics,
                difficulty: Difficulty::Hard,
            },
            Question {
                id: 17,
                question: "PKM机枪最适合在什么场景下使用？".to_string(),
                options: [
                    "室内近战".to_string(),
                    "开阔地带火力压制".to_string(),
                    "潜行暗杀".to_string(),
                    "快速突击".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Weapons,
                difficulty: Difficulty::Hard,
            },
            Question {
                id: 18,
                question: "在学校地图中，哪个位置最容易形成战术瓶颈？".to_string(),
                options: [
                    "操场区域".to_string(),
                    "教学楼走廊".to_string(),
                    "图书馆".to_string(),
                    "食堂".to_string(),
                ],
                correct_answer: 1,
                category: QuestionCategory::Maps,
                difficulty: Difficulty::Hard,
            },
        ]
    }

    pub fn get_question_by_id(id: u32) -> Option<Question> {
        Self::get_all_questions().into_iter().find(|q| q.id == id)
    }

    pub fn get_questions_by_category(category: QuestionCategory) -> Vec<Question> {
        Self::get_all_questions()
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
            .collect()
    }

    pub fn get_questions_by_difficulty(difficulty: Difficulty) -> Vec<Question> {
        Self::get_all_questions()
            .into_iter()
            .filter(|q| {
                matches!(
                    (&q.difficulty, &difficulty),
                    (Difficulty::Easy, Difficulty::Easy)
                        | (Difficulty::Medium, Difficulty::Medium)
                        | (Difficulty::Hard, Difficulty::Hard)
                )
            })
            .collect()
    }
}

// 题目生成器
pub struct QuestionGenerator;

impl QuestionGenerator {
    pub fn get_random_question() -> QuestionResponse {
        let mut rng = rand::thread_rng();
        let questions = QuestionBank::get_all_questions();
        let question = questions.choose(&mut rng).unwrap().clone();

        Self::format_question_response(question)
    }

    pub fn get_random_question_by_category(category: QuestionCategory) -> Option<QuestionResponse> {
        let mut rng = rand::thread_rng();
        let questions = QuestionBank::get_questions_by_category(category);

        if questions.is_empty() {
            return None;
        }

        let question = questions.choose(&mut rng).unwrap().clone();
        Some(Self::format_question_response(question))
    }

    pub fn get_random_question_by_difficulty(difficulty: Difficulty) -> Option<QuestionResponse> {
        let mut rng = rand::thread_rng();
        let questions = QuestionBank::get_questions_by_difficulty(difficulty);

        if questions.is_empty() {
            return None;
        }

        let question = questions.choose(&mut rng).unwrap().clone();
        Some(Self::format_question_response(question))
    }

    pub fn check_answer(question_id: u32, selected_option: u8) -> Option<AnswerResult> {
        let question = QuestionBank::get_question_by_id(question_id)?;

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

        Some(AnswerResult {
            is_correct,
            correct_answer: question.correct_answer,
            correct_option,
            explanation: Self::get_explanation(&question),
        })
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
pub async fn get_random_question() -> ResponseJson<serde_json::Value> {
    let question_response = QuestionGenerator::get_random_question();
    ResponseJson(json!(question_response))
}

pub async fn get_question_by_category(
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

pub async fn get_question_by_difficulty(
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

pub async fn submit_answer(
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