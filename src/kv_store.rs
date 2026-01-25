//! A simple in-memory key-value store with JSON serialization and disk persistence.

use std::{collections::HashMap, sync::RwLock};
use std::fs;
use crate::wal::{Wal, WalError, WalEntry};

/// An in-memory key-value store backed by a HashMap with JSON persistence support.
///
/// # Examples
///
/// ```
/// use rust_kv_store::kv_store::KvStore;
///
/// let store = KvStore::new("target/test_set_and_get.wal").unwrap();
/// let _ = store.set("key".to_string(), "value".to_string());
/// assert_eq!(store.get("key"), Some("value".to_string()));
/// ```

pub struct KvStore {
    map: RwLock<HashMap<String, String>>,
    wal: Wal
}

impl KvStore {
    /// Creates a new empty KvStore.
    pub fn new(wal_path: &str) -> Result<Self, String> {
        let wal = Wal::open(wal_path)?;

        let map = RwLock::new(HashMap::new());

        let store = KvStore { map, wal };

        store.wal.replay(&mut store.map.write().unwrap())?;

        Ok(store)  
    }

    /// Retrieves the value associated with the given key.
    pub fn get(&self, key: &str) -> Option<String> {
        self.map.read().unwrap().get(key).cloned()
    }

    /// Stores a key-value pair. Overwrites any existing value.
    pub fn set(&self, key: String, value: String) -> Result<(), WalError> {
        self.wal.append(WalEntry::Set(key.clone(), value.clone()))?;

        self.map.write().unwrap().insert(key, value);

        Ok(())
    }

    /// Removes a key-value pair and returns the deleted value.
    pub fn delete(&self, key: &str) -> Result<Option<String>, WalError> {
        self.wal.append(WalEntry::Delete(key.to_string()))?;

        Ok(self.map.write().unwrap().remove(key))
    }

    /// Removes all key-value pairs from the store.
    pub fn clear(&self) -> Result<(), WalError> {
        self.wal.append(WalEntry::Clear)?;

        self.map.write().unwrap().clear();

        Ok(())
    }

    /// Returns the number of key-value pairs in the store.
    pub fn len(&self) -> usize {
        self.map.read().unwrap().len()
    }

    /// Saves the store to a JSON file.
    ///
    /// # Arguments
    /// * `path` - File path where the store will be written
    ///
    /// # Returns
    /// Ok(()) if successful, Err with message describing serialization/IO errors
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let map = self.map.read().unwrap();

        let json = serde_json::to_string_pretty(&*map)
            .map_err(|e| format!("Serialize error: {}", e))?;
        
        fs::write(path, json)
            .map_err(|e| format!("Write error: {}", e))?;

        self.wal.truncate().unwrap();

        Ok(())
    }

    /// Loads a KvStore from a JSON file.
    ///
    /// # Arguments
    /// * `path` - File path where the store will be loaded
    ///
    /// # Returns
    /// Ok(KvStore) if successful, Err with message describing serialization/IO errors
    pub fn load_from_file(path: &str, wal_path: &str) -> Result<Self, String> {
        let json = fs::read_to_string(path)
            .map_err(|e| format!("Read error: {}", e))?;

        let map: HashMap<String, String> = 
            serde_json::from_str(&json)
                .map_err(|e| format!("Deserialize error: {}", e))?;

        let wal = Wal::open(wal_path)?;

        Ok(KvStore { map: RwLock::new(map), wal })
    }
}