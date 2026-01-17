use rust_kv_store::kv_store::KvStore;

#[test]
fn test_set_and_get() {
    let mut store = KvStore::new();
    store.set("key".to_string(), "value".to_string());
    assert_eq!(store.get("key"), Some("value".to_string()));
}

#[test]
fn test_get_missing_key() {
    let store = KvStore::new();
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_set_existing_key() {
    let mut store = KvStore::new();
    store.set("key".to_string(), "value1".to_string());
    assert_eq!(store.get("key"), Some("value1".to_string()));
    store.set("key".to_string(), "value2".to_string());
    assert_eq!(store.get("key"), Some("value2".to_string()));
}

#[test]
fn test_set_empty_string_key_and_value() {
    let mut store = KvStore::new();
    store.set("".to_string(), "".to_string());
    assert_eq!(store.get(""), Some("".to_string()));
}

#[test]
fn test_unicode_keys_and_values() {
    let mut store = KvStore::new();
    store.set("食".to_string(), "食".to_string());
    assert_eq!(store.get("食"), Some("食".to_string()));
}

#[test]
fn test_delete() {
    let mut store = KvStore::new();
    store.set("key".to_string(), "value".to_string());

    let deleted = store.delete("key");
    assert_eq!(deleted, Some("value".to_string()));
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_delete_missing_key() {
    let mut store = KvStore::new();
    let deleted = store.delete("key");
    assert_eq!(deleted, None);
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_clear() {
    let mut store = KvStore::new();
    store.set("key1".to_string(), "value1".to_string());
    store.set("key2".to_string(), "value2".to_string());
    store.clear();
    assert_eq!(store.get("key1"), None);
    assert_eq!(store.get("key2"), None);
}

#[test]
fn test_clear_empty_store() {
    let mut store = KvStore::new();
    store.clear();
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_len() {
    let mut store = KvStore::new();
    store.set("key1".to_string(), "value1".to_string());
    store.set("key2".to_string(), "value2".to_string());
    assert_eq!(store.len(), 2);
    store.clear();
    assert_eq!(store.len(), 0);
}

#[test]
fn test_persistence() {
    let mut store = KvStore::new();
    store.set("key1".to_string(), "value1".to_string());
    store.set("key2".to_string(), "value2".to_string());
    store.save_to_file("target/test_persistence.json").expect("File is saved successfully.");
    store.clear();
    assert_eq!(store.len(), 0);
    store = KvStore::load_from_file("target/test_persistence.json").expect("File is loaded successfully.");
    assert_eq!(store.len(), 2);
    assert_eq!(store.get("key1"), Some("value1".to_string()));
    assert_eq!(store.get("key2"), Some("value2".to_string()));
}