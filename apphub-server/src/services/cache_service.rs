//! 缓存服务

use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// 黑名单缓存
static BLACKLIST_CACHE: Lazy<Mutex<HashMap<String, i16>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 获取黑名单缓存
pub fn get_blacklist_cache() -> HashMap<String, i16> {
    let cache = BLACKLIST_CACHE.lock().unwrap();
    cache.clone()
}

/// 更新黑名单缓存
pub fn update_blacklist_cache(blacklist: HashMap<String, i16>) {
    let mut cache = BLACKLIST_CACHE.lock().unwrap();
    *cache = blacklist;
}

/// 清空黑名单缓存
pub fn clear_blacklist_cache() {
    let mut cache = BLACKLIST_CACHE.lock().unwrap();
    cache.clear();
}
