use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Cache item with expiration logic
#[derive(Debug)]
struct ExpiringCacheItem<T> {
    value: T,
    created: Instant,
    expires_after: Duration,
}

impl<T> ExpiringCacheItem<T> {
    fn new(value: T, expires_after: Duration) -> Self {
        Self {
            value,
            created: Instant::now(),
            expires_after,
        }
    }

    fn is_expired(&self) -> bool {
        self.created.elapsed() >= self.expires_after
    }
}

// Expiring cache implementation
pub struct ExpiringCache<TKey, TValue> {
    cache: Arc<Mutex<HashMap<TKey, ExpiringCacheItem<TValue>>>>,
}

impl<TKey: Eq + std::hash::Hash + Clone, TValue> ExpiringCache<TKey, TValue> {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn store(&self, key: TKey, value: TValue, expires_after: Duration) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, ExpiringCacheItem::new(value, expires_after));
    }

    pub fn get(&self, key: &TKey) -> Option<TValue>
    where
        TValue: Clone,
    {
        let mut cache = self.cache.lock().unwrap();
        if let Some(item) = cache.get(key) {
            if item.is_expired() {
                cache.remove(key);
                return None;
            }
            return Some(item.value.clone());
        }
        None
    }
}
