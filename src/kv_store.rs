//! A simple in-memory key-value store for learning Rust systems programming.

use std::collections::HashMap;

/// A thread-unsafe, in-memory key-value store backed by a HashMap.
///
/// # Examples
///
/// ```
/// use rust_kv_store::kv_store::KvStore;
///
/// let mut store = KvStore::new();
/// store.set("key".to_string(), "value".to_string());
/// assert_eq!(store.get("key"), Some("value".to_string()));
/// ```
pub struct KvStore {
    map: HashMap<String, String>
}

impl KvStore {
    /// Creates a new empty KvStore.
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new()
        }
    }

    /// Retrieves the value associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key to look up
    ///
    /// # Returns
    /// Some(value) if the key exists, None otherwise
    pub fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }

    /// Stores a key-value pair. Overwrites any existing value.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Removes a key-value pair and returns the deleted value.
    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }

    /// Removes all key-value pairs from the store.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Returns the number of key-value pairs in the store.
    pub fn len(&self) -> usize {
        self.map.len()
    }
}