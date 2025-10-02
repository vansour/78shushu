use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[allow(dead_code)]
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StaticConfig {
    pub directory: String,
    pub precompressed: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub cache: CacheConfig,
    #[serde(rename = "static")]
    pub static_files: StaticConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
                log_level: "info".to_string(),
            },
            cache: CacheConfig {
                enabled: true,
                ttl: 3600,
            },
            static_files: StaticConfig {
                directory: "static".to_string(),
                precompressed: true,
            },
        }
    }
}

pub static CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    let config = config::Config::builder()
        .add_source(config::File::with_name("config/config").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .and_then(|c| c.try_deserialize::<AppConfig>())
        .unwrap_or_else(|err| {
            tracing::warn!("无法加载配置文件，使用默认配置: {}", err);
            AppConfig::default()
        });

    tracing::info!("配置已加载: 端口={}, 缓存={}", config.server.port, config.cache.enabled);
    Arc::new(config)
});

pub fn get_config() -> &'static AppConfig {
    &CONFIG
}
