use std::collections::HashMap;
/// a diy kv store
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// create a kv store
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }
    /// set kv pair
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    /// get value by key
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
    /// remove kv pair by key
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
