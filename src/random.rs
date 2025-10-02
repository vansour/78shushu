use crate::error::{AppError, AppResult};
use axum::extract::Query;
use axum::response::Json as ResponseJson;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RandomItem {
    name: String,
    image: String,
}

#[derive(Debug, Deserialize)]
struct ItemData {
    #[serde(default)]
    #[allow(dead_code)]
    categories: serde_json::Value, // 忽略 categories 字段
    items: Vec<RandomItem>,
}

#[derive(Debug, Deserialize)]
struct RandomData {
    maps: ItemData,
    operators: ItemData,
    primary_weapons: ItemData,
    helmets: ItemData,
    armors: ItemData,
}

#[derive(Debug, Deserialize)]
pub struct LoadoutFilters {
    #[serde(default)]
    pub classified_only: bool, // 只玩机密/绝密
    #[serde(default)]
    pub exclude_pistols: bool, // 排除手枪
}

// 通用随机选择函数
fn random_choice<T: Clone>(items: &[T]) -> AppResult<T> {
    let mut rng = rand::thread_rng();
    items
        .choose(&mut rng)
        .cloned()
        .ok_or_else(|| AppError::RandomGenerationError("无法从空集合中选择项目".to_string()))
}

// 从JSON文件加载随机数据（带缓存）
fn load_random_data() -> RandomData {
    tracing::debug!(target: "random", "开始加载随机数据配置");

    let config = crate::config::get_config();
    
    // 如果启用缓存，先尝试从缓存获取
    if config.cache.enabled {
        if let Some(cached_content) = crate::cache::RANDOM_DATA_CACHE.get("random_data") {
            if let Ok(data) = serde_json::from_str::<RandomData>(&cached_content) {
                tracing::debug!(target: "random", "从缓存加载随机数据");
                return data;
            }
        }
    }

    if let Ok(content) = std::fs::read_to_string("static/random.json") {
        if let Ok(data) = serde_json::from_str::<RandomData>(&content) {
            tracing::info!(target: "random", "成功加载随机数据配置");
            
            // 保存到缓存
            if config.cache.enabled {
                crate::cache::RANDOM_DATA_CACHE.set("random_data".to_string(), content);
                tracing::debug!(target: "random", "随机数据已缓存");
            }
            
            return data;
        } else {
            tracing::warn!(target: "random", "解析 random.json 失败，使用默认数据");
        }
    } else {
        tracing::warn!(target: "random", "读取 random.json 失败，使用默认数据");
    }

    // 默认数据作为fallback
    RandomData {
        maps: ItemData {
            categories: serde_json::json!({}),
            items: vec![RandomItem {
                name: "零号大坝（常规）".to_string(),
                image: "/static/picture/random/maps/default.jpg".to_string(),
            }],
        },
        operators: ItemData {
            categories: serde_json::json!({}),
            items: vec![RandomItem {
                name: "红狼".to_string(),
                image: "/static/picture/random/operators/default.jpg".to_string(),
            }],
        },
        primary_weapons: ItemData {
            categories: serde_json::json!({}),
            items: vec![RandomItem {
                name: "M4A1突击步枪".to_string(),
                image: "/static/picture/random/weapons/default.jpg".to_string(),
            }],
        },
        helmets: ItemData {
            categories: serde_json::json!({}),
            items: vec![RandomItem {
                name: "H01 战术头盔".to_string(),
                image: "/static/picture/random/helmets/default.jpg".to_string(),
            }],
        },
        armors: ItemData {
            categories: serde_json::json!({}),
            items: vec![RandomItem {
                name: "轻型防弹衣".to_string(),
                image: "/static/picture/random/armors/default.jpg".to_string(),
            }],
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadoutItem {
    pub name: String,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomLoadout {
    pub map: LoadoutItem,
    pub operator: LoadoutItem,
    pub primary_weapon: LoadoutItem,
    pub helmet: LoadoutItem,
    pub armor: LoadoutItem,
}

// 随机装备生成器
pub struct RandomGenerator;

impl RandomGenerator {
    pub fn generate_loadout(filters: &LoadoutFilters) -> AppResult<RandomLoadout> {
        tracing::info!(target: "random", "开始生成随机装备，过滤条件 - 机密/绝密: {}, 排除手枪: {}", 
                      filters.classified_only, filters.exclude_pistols);

        let data = load_random_data();

        // 根据过滤条件筛选地图
        let map_items: Vec<RandomItem> = if filters.classified_only {
            data.maps
                .items
                .into_iter()
                .filter(|item| item.name.contains("机密") || item.name.contains("绝密"))
                .collect()
        } else {
            data.maps.items
        };

        if map_items.is_empty() {
            return Err(AppError::RandomGenerationError(
                "没有符合条件的地图".to_string(),
            ));
        }

        // 根据过滤条件筛选主武器
        let weapon_items: Vec<RandomItem> = if filters.exclude_pistols {
            data.primary_weapons
                .items
                .into_iter()
                .filter(|item| {
                    // 排除手枪类主武器（虽然主武器中一般没有手枪，但为了保险还是做过滤）
                    !item.name.contains("手枪")
                        && !item.name.contains("M1911")
                        && !item.name.contains("G17")
                        && !item.name.contains("G18")
                        && !item.name.contains("93R")
                        && !item.name.contains("沙漠之鹰")
                        && !item.name.contains(".357")
                        && !item.name.contains("QSZ92G")
                })
                .collect()
        } else {
            data.primary_weapons.items
        };

        if weapon_items.is_empty() {
            return Err(AppError::RandomGenerationError(
                "没有符合条件的武器".to_string(),
            ));
        }

        let map_item = random_choice(&map_items)?;
        let operator_item = random_choice(&data.operators.items)?;
        let primary_weapon_item = random_choice(&weapon_items)?;
        let helmet_item = random_choice(&data.helmets.items)?;
        let armor_item = random_choice(&data.armors.items)?;

        let loadout = RandomLoadout {
            map: LoadoutItem {
                name: map_item.name.clone(),
                image: map_item.image.clone(),
            },
            operator: LoadoutItem {
                name: operator_item.name.clone(),
                image: operator_item.image.clone(),
            },
            primary_weapon: LoadoutItem {
                name: primary_weapon_item.name.clone(),
                image: primary_weapon_item.image.clone(),
            },
            helmet: LoadoutItem {
                name: helmet_item.name.clone(),
                image: helmet_item.image.clone(),
            },
            armor: LoadoutItem {
                name: armor_item.name.clone(),
                image: armor_item.image.clone(),
            },
        };

        tracing::info!(target: "random", "成功生成随机装备 - 地图: {}, 干员: {}", 
                      loadout.map.name, loadout.operator.name);

        Ok(loadout)
    }
}

// 随机装备相关接口
pub async fn generate_full_loadout(
    Query(filters): Query<LoadoutFilters>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    tracing::info!(target: "random", "API调用: 生成完整装备");
    let loadout = RandomGenerator::generate_loadout(&filters)?;
    tracing::debug!(target: "random", "返回装备配置: {:?}", loadout);
    Ok(ResponseJson(json!(loadout)))
}
