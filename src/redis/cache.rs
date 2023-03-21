use std::collections::HashMap;
use std::hash::Hash;
use std::time;
use thiserror::Error;

struct Item<Value> {
    value: Value,
    expires_at: Option<time::Instant>,
}

#[derive(Error, PartialEq, Debug)]
pub enum CacheError {
    #[error("Key not found")]
    Missing,
    #[error("Key has expired")]
    Expired,
}

pub struct Cache<K: Sized, V> {
    items: HashMap<K, Item<V>>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Sized,
{
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn fetch(&self, k: &K) -> Option<&Item<V>> {
        self.items.get(k)
    }

    pub fn value(&mut self, k: &K) -> Result<&V, CacheError> {
        if self.del_if_expired(k) {
            return Err(CacheError::Expired);
        }
        self.fetch(k).map(|i| &i.value).ok_or(CacheError::Missing)
    }

    pub fn put(&mut self, k: K, v: V, t: Option<time::Instant>) {
        let item = Item {
            value: v,
            expires_at: t,
        };
        self.items.insert(k, item);
    }

    fn del(&mut self, k: &K) {
        self.items.remove(k);
    }

    fn del_if_expired(&mut self, k: &K) -> bool {
        let is_expired = self.items.get(k).map(|i| i.is_expired()).unwrap_or(false);
        if is_expired {
            self.del(k);
        }
        is_expired
    }
}

impl<T> Item<T> {
    fn is_expired(&self) -> bool {
        self.expires_at
            .map(|t| t <= time::Instant::now())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let mut cache = Cache::new();
        cache.put("key", 42, None);
        assert_eq!(cache.value(&"key"), Ok(&42));
    }

    #[test]
    fn test_value_miss() {
        let mut cache = Cache::new();
        cache.put("key", 42, Some(time::Instant::now()));
        assert_eq!(cache.value(&"key"), Err(CacheError::Missing));
    }
}
