use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Map {
    #[serde(rename = "海岸线")]
    Coastline,
    #[serde(rename = "农场")]
    Farm,
    #[serde(rename = "总部")]
    Headquarters,
    #[serde(rename = "港口")]
    Harbor,
    #[serde(rename = "边境")]
    Border,
    #[serde(rename = "工厂")]
    Factory,
    #[serde(rename = "学校")]
    School,
    #[serde(rename = "医院")]
    Hospital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "突击兵")]
    Assault,
    #[serde(rename = "工程师")]
    Engineer,
    #[serde(rename = "支援兵")]
    Support,
    #[serde(rename = "狙击手")]
    Recon,
    #[serde(rename = "医疗兵")]
    Medic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimaryWeapon {
    // 突击步枪
    #[serde(rename = "AK-74")]
    AK74,
    #[serde(rename = "M4A1")]
    M4A1,
    #[serde(rename = "AK-12")]
    AK12,
    #[serde(rename = "M416")]
    M416,
    #[serde(rename = "SCAR-L")]
    SCARL,
    
    // 冲锋枪
    #[serde(rename = "MP5")]
    MP5,
    #[serde(rename = "UMP45")]
    UMP45,
    #[serde(rename = "Vector")]
    Vector,
    
    // 狙击枪
    #[serde(rename = "AWP")]
    AWP,
    #[serde(rename = "M24")]
    M24,
    #[serde(rename = "SV-98")]
    SV98,
    
    // 机枪
    #[serde(rename = "M249")]
    M249,
    #[serde(rename = "PKM")]
    PKM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Helmet {
    #[serde(rename = "轻型头盔")]
    Light,
    #[serde(rename = "中型头盔")]
    Medium,
    #[serde(rename = "重型头盔")]
    Heavy,
    #[serde(rename = "战术头盔")]
    Tactical,
    #[serde(rename = "防弹头盔")]
    Bulletproof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Armor {
    #[serde(rename = "轻型护甲")]
    Light,
    #[serde(rename = "中型护甲")]
    Medium,
    #[serde(rename = "重型护甲")]
    Heavy,
    #[serde(rename = "战术背心")]
    TacticalVest,
    #[serde(rename = "防弹衣")]
    BodyArmor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomLoadout {
    pub map: Map,
    pub operator: Operator,
    pub primary_weapon: PrimaryWeapon,
    pub helmet: Helmet,
    pub armor: Armor,
}

impl Map {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Coastline,
            Self::Farm,
            Self::Headquarters,
            Self::Harbor,
            Self::Border,
            Self::Factory,
            Self::School,
            Self::Hospital,
        ]
    }
}

impl Operator {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Assault,
            Self::Engineer,
            Self::Support,
            Self::Recon,
            Self::Medic,
        ]
    }
}

impl PrimaryWeapon {
    pub fn all() -> Vec<Self> {
        vec![
            Self::AK74,
            Self::M4A1,
            Self::AK12,
            Self::M416,
            Self::SCARL,
            Self::MP5,
            Self::UMP45,
            Self::Vector,
            Self::AWP,
            Self::M24,
            Self::SV98,
            Self::M249,
            Self::PKM,
        ]
    }
}

impl Helmet {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Light,
            Self::Medium,
            Self::Heavy,
            Self::Tactical,
            Self::Bulletproof,
        ]
    }
}

impl Armor {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Light,
            Self::Medium,
            Self::Heavy,
            Self::TacticalVest,
            Self::BodyArmor,
        ]
    }
}

// 三角洲高考相关数据结构
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

// 音乐播放器相关数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub songs: Vec<Song>,
}