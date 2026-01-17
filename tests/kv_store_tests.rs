use rust_kv_store::kv_store::KvStore;

#[test]
fn test_set_and_get() {
    let mut store = KvStore::new();
    store.set("key".to_string(), "value".to_string());
    assert_eq!(store.get("key"), Some("value".to_string()));
}

#[test]
fn test_delete() {
    let mut store = KvStore::new();
    store.set("key".to_string(), "value".to_string());

    let deleted = store.delete("key");
    assert_eq!(deleted, Some("value".to_string()));
    assert_eq!(store.get("key"), None);
}