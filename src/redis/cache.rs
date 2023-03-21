use std::collections::HashMap;
use std::hash::Hash;
use std::time;

struct Item<Value> {
    _expires_at: time::Instant,
    value: Value,
}

impl<T> Item<T> {
    fn new(value: T) -> Self {
        Self {
            _expires_at: time::Instant::now(),
            value,
        }
    }
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
        self.items.get(k).map(|i| &i.value)
    }

    pub fn put(&mut self, k: K, v: V) {
        let item = Item::new(v);
        self.items.insert(k, item);
    }

    pub fn del(&mut self, k: &K) {
        self.items.remove(k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut cache = Cache::new();
        cache.put("key", 42);
        assert_eq!(cache.get(&"key"), Some(&42));
    }

    #[test]
    fn test_del() {
        let mut cache = Cache::new();
        cache.put("key", 42);
        assert_eq!(cache.get(&"key"), Some(&42));
        cache.del(&"key");
        assert_eq!(cache.get(&"key"), None);
    }
}
