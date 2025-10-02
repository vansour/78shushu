use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

pub struct Cache<T: Clone> {
    store: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    ttl: Duration,
}

impl<T: Clone> Cache<T> {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let store = self.store.read().ok()?;
        let entry = store.get(key)?;
        
        if entry.expires_at > Instant::now() {
            Some(entry.data.clone())
        } else {
            None
        }
    }

    pub fn set(&self, key: String, data: T) {
        if let Ok(mut store) = self.store.write() {
            store.insert(
                key,
                CacheEntry {
                    data,
                    expires_at: Instant::now() + self.ttl,
                },
            );
        }
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        if let Ok(mut store) = self.store.write() {
            store.clear();
        }
    }

    #[allow(dead_code)]
    pub fn remove_expired(&self) {
        if let Ok(mut store) = self.store.write() {
            let now = Instant::now();
            store.retain(|_, entry| entry.expires_at > now);
        }
    }
}

// 全局缓存实例
pub static RANDOM_DATA_CACHE: Lazy<Cache<String>> = Lazy::new(|| {
    let config = crate::config::get_config();
    Cache::new(config.cache.ttl)
});

#[allow(dead_code)]
pub static QUESTION_DATA_CACHE: Lazy<Cache<String>> = Lazy::new(|| {
    let config = crate::config::get_config();
    Cache::new(config.cache.ttl)
});

#[allow(dead_code)]
pub static MUSIC_DATA_CACHE: Lazy<Cache<String>> = Lazy::new(|| {
    let config = crate::config::get_config();
    Cache::new(config.cache.ttl)
});
