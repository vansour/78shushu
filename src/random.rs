use crate::error::{AppError, AppResult};
use axum::response::Json as ResponseJson;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct RandomData {
    maps: Vec<String>,
    operators: Vec<String>,
    primary_weapons: Vec<String>,
    helmets: Vec<String>,
    armors: Vec<String>,
}

// 通用随机选择函数
fn random_choice<T: Clone>(items: &[T]) -> AppResult<T> {
    let mut rng = rand::thread_rng();
    items
        .choose(&mut rng)
        .cloned()
        .ok_or_else(|| AppError::RandomGenerationError("无法从空集合中选择项目".to_string()))
}

// 从JSON文件加载随机数据
fn load_random_data() -> RandomData {
    if let Ok(content) = std::fs::read_to_string("static/random.json") {
        if let Ok(data) = serde_json::from_str::<RandomData>(&content) {
            return data;
        }
    }

    // 默认数据作为fallback
    RandomData {
        maps: vec!["零号大坝（常规）".to_string()],
        operators: vec!["红狼".to_string()],
        primary_weapons: vec!["M4A1突击步枪".to_string()],
        helmets: vec!["H01 战术头盔".to_string()],
        armors: vec!["轻型防弹衣".to_string()],
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomLoadout {
    pub map: String,
    pub operator: String,
    pub primary_weapon: String,
    pub helmet: String,
    pub armor: String,
}

// 随机装备生成器
pub struct RandomGenerator;

impl RandomGenerator {
    pub fn generate_loadout() -> AppResult<RandomLoadout> {
        let data = load_random_data();

        Ok(RandomLoadout {
            map: random_choice(&data.maps)?,
            operator: random_choice(&data.operators)?,
            primary_weapon: random_choice(&data.primary_weapons)?,
            helmet: random_choice(&data.helmets)?,
            armor: random_choice(&data.armors)?,
        })
    }
}

// 随机装备相关接口
pub async fn generate_full_loadout() -> Result<ResponseJson<serde_json::Value>, AppError> {
    let loadout = RandomGenerator::generate_loadout()?;
    Ok(ResponseJson(json!(loadout)))
}
