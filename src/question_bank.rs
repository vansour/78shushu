use crate::models::*;

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
