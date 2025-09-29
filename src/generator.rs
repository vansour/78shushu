use crate::models::*;
use crate::question_bank::QuestionBank;
use rand::seq::SliceRandom;

pub struct RandomGenerator;

impl RandomGenerator {
    pub fn generate_loadout() -> RandomLoadout {
        let mut rng = rand::thread_rng();

        let maps = Map::all();
        let operators = Operator::all();
        let weapons = PrimaryWeapon::all();
        let helmets = Helmet::all();
        let armors = Armor::all();

        RandomLoadout {
            map: maps.choose(&mut rng).unwrap().clone(),
            operator: operators.choose(&mut rng).unwrap().clone(),
            primary_weapon: weapons.choose(&mut rng).unwrap().clone(),
            helmet: helmets.choose(&mut rng).unwrap().clone(),
            armor: armors.choose(&mut rng).unwrap().clone(),
        }
    }

    pub fn generate_map() -> Map {
        let mut rng = rand::thread_rng();
        let maps = Map::all();
        maps.choose(&mut rng).unwrap().clone()
    }

    pub fn generate_operator() -> Operator {
        let mut rng = rand::thread_rng();
        let operators = Operator::all();
        operators.choose(&mut rng).unwrap().clone()
    }

    pub fn generate_weapon() -> PrimaryWeapon {
        let mut rng = rand::thread_rng();
        let weapons = PrimaryWeapon::all();
        weapons.choose(&mut rng).unwrap().clone()
    }

    pub fn generate_helmet() -> Helmet {
        let mut rng = rand::thread_rng();
        let helmets = Helmet::all();
        helmets.choose(&mut rng).unwrap().clone()
    }

    pub fn generate_armor() -> Armor {
        let mut rng = rand::thread_rng();
        let armors = Armor::all();
        armors.choose(&mut rng).unwrap().clone()
    }
}

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
