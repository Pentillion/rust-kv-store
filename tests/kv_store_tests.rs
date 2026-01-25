use std::{sync::Arc, thread};

use rust_kv_store::kv_store::KvStore;

#[test]
fn test_set_and_get() {
    let store = new_test_store("target/test_set_and_get.wal");
    store.set("key".to_string(), "value".to_string()).expect("Failed to set key");
    assert_eq!(store.get("key"), Some("value".to_string()));
}

#[test]
fn test_get_missing_key() {
    let store = new_test_store("target/test_get_missing_key.wal");
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_set_existing_key() {
    let store = new_test_store("target/test_set_existing_key.wal");
    store.set("key".to_string(), "value1".to_string()).expect("Failed to set key");
    assert_eq!(store.get("key"), Some("value1".to_string()));
    store.set("key".to_string(), "value2".to_string()).expect("Failed to set key");
    assert_eq!(store.get("key"), Some("value2".to_string()));
}

#[test]
fn test_set_empty_string_key_and_value() {
    let store = new_test_store("target/test_set_empty_string_key_and_value.wal");
    store.set("".to_string(), "".to_string()).expect("Failed to set key");
    assert_eq!(store.get(""), Some("".to_string()));
}

#[test]
fn test_unicode_keys_and_values() {
    let store = new_test_store("target/test_unicode_keys_and_values.wal");
    store.set("食".to_string(), "食".to_string()).expect("Failed to set key");
    assert_eq!(store.get("食"), Some("食".to_string()));
}

#[test]
fn test_delete() {
    let store = new_test_store("target/test_delete.wal");
    store.set("key".to_string(), "value".to_string()).expect("Failed to set key");

    let deleted = store.delete("key");
    assert_eq!(deleted.unwrap(), Some("value".to_string()));
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_delete_missing_key() {
    let store = new_test_store("target/test_delete_missing_key.wal");
    let deleted = store.delete("key");
    assert_eq!(deleted.unwrap(), None);
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_clear() {
    let store = new_test_store("target/test_clear.wal");
    store.set("key1".to_string(), "value1".to_string()).expect("Failed to set key");
    store.set("key2".to_string(), "value2".to_string()).expect("Failed to set key");
    store.clear().expect("Failed to set clear");
    assert_eq!(store.get("key1"), None);
    assert_eq!(store.get("key2"), None);
}

#[test]
fn test_clear_empty_store() {
    let store = new_test_store("target/test_clear_empty_store.wal");
    store.clear().expect("Failed to set clear");
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_len() {
    let store = new_test_store("target/test_len.wal");
    store.set("key1".to_string(), "value1".to_string()).expect("Failed to set key");
    store.set("key2".to_string(), "value2".to_string()).expect("Failed to set key");
    assert_eq!(store.len(), 2);
    store.clear().expect("Failed to set clear");
    assert_eq!(store.len(), 0);
}

#[test]
fn test_persistence() {
    let persistence_path = "target/test_persistence.json";
    let wal_path = "target/test_persistence.wal";
    let store = new_test_store(wal_path);
    store.set("key1".to_string(), "value1".to_string()).expect("Failed to set key");
    store.set("key2".to_string(), "value2".to_string()).expect("Failed to set key");
    store.save_to_file(persistence_path).expect("File is saved successfully.");
    store.clear().expect("Failed to clear");
    assert_eq!(store.len(), 0);
    let store = KvStore::load_from_file(persistence_path, wal_path).expect("File is loaded successfully.");
    assert_eq!(store.len(), 2);
    assert_eq!(store.get("key1"), Some("value1".to_string()));
    assert_eq!(store.get("key2"), Some("value2".to_string()));
}

#[test]
fn test_concurrent_writes_and_reads() {
    let store = Arc::new(KvStore::new("target/test_concurrent_writes_and_reads.wal").unwrap());

    let mut handles = vec![];

    for i in 0..10 {
        let store_clone = Arc::clone(&store);
        handles.push(thread::spawn(move || {
            for j in 0..100 {
                let key = format!("key_{}_{}", i, j);
                let value = format!("value_{}_{}", i, j);
                let _ = store_clone.set(key, value);
            }
        }));
    }

    for _ in 0..5 {
        let store_clone = Arc::clone(&store);
        handles.push(thread::spawn(move || {
            for j in 0..100 {
                let key = format!("key_0_{}", j);
                store_clone.get(&key);
            }
        }));  
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for i in 0..10 {
        for j in 0..100 {
            let key = format!("key_{}_{}", i, j);
            let value = format!("value_{}_{}", i, j);
            assert_eq!(store.get(&key), Some(value));
        }
    }
}

#[test]
fn test_wal_recovery() {
    let wal_path = "target/test_wal_recovery.wal";

    let store = new_test_store(wal_path);
    store.set("a".into(), "1".into()).unwrap();
    store.set("b".into(), "2".into()).unwrap();

    drop(store);

    let recovered = KvStore::new(wal_path).unwrap();
    assert_eq!(recovered.get("a"), Some("1".into()));
    assert_eq!(recovered.get("b"), Some("2".into()));
}

fn new_test_store(wal_path: &str) -> KvStore {
    std::fs::remove_file(wal_path).ok();
    KvStore::new(wal_path).expect("Failed to create KvStore")
}