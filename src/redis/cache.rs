use std::collections::HashMap;
use std::hash::Hash;
use std::time;

struct Item<Value> {
    value: Value,
    expires_at: Option<time::Instant>,
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

    pub fn get(&mut self, k: &K) -> Option<&V> {
        self.del_if_expired(k);
        self.items.get(k).map(|i| &i.value)
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

    fn del_if_expired(&mut self, k: &K) {
        let timeout = self.items.get(k).and_then(|i| i.expires_at);
        if let Some(timeout) = timeout {
            if timeout < time::Instant::now() {
                self.del(k);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut cache = Cache::new();
        cache.put("key", 42, None);
        assert_eq!(cache.get(&"key"), Some(&42));
    }

    #[test]
    fn test_del() {
        let mut cache = Cache::new();
        cache.put("key", 42, None);
        assert_eq!(cache.get(&"key"), Some(&42));
        cache.del(&"key");
        assert_eq!(cache.get(&"key"), None);
    }
}
