use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}