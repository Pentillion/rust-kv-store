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
        // TODO: implement
        unimplemented!()
    }

    pub fn set(&self, key: String, value: String) {
        // TODO: implement
        unimplemented!()
    }

    pub fn delete(&self, key: &str) -> Option<String> {
        // TODO: implement
        unimplemented!()
    }
}