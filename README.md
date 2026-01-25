# rust-kv-store

An in-memory key-value store with disk persistence via JSON serialization and crash durability using a Write-Ahead Log (WAL).
Data lives in RAM but can be manually saved and loaded from disk, with WAL ensuring that operations are not lost in case of a crash.

## Motivation

This project is meant to learn systems programming concepts in Rust, including:

- Ownership and memory management
- Correctness under failure
- Concurrency
- Persistent storage

---

### Data Structure (Version 3)

Internally, the store is backed by a `RwLock<HashMap<String, String>>`:

```rust
use std::{collections::HashMap, sync::RwLock};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct KvStore {
    map: RwLock<HashMap<String, String>>,
    wal: Wal
}
```

---

### API (Version 3)

The in-memory key-value store exposes the following methods:

- `fn get(&self, key: &str) -> Option<String>`

Returns the value associated with a key, or `None` if the key does not exist.

- `fn set(&mut self, key: String, value: String) -> Result<(), WalError>`

Stores a key-value pair. Overwrites any existing value.

- `fn delete(&mut self, key: &str) -> Result<Option<String>, WalError>`

Removes a key-value pair. Returns the deleted value if it existed.

- `fn clear(&mut self) -> Result<(), WalError>`

Removes all key-value pairs from the store.

- `fn len(&self) -> usize`

Returns the number of key-value pairs in the store.

- `fn save_to_file(&self, path: &str) -> Result<(), String>`

Serializes store data to json and saves the file on disk.

- `fn load_from_file(path: &str, wal_path: &str) -> Result<KvStore, String>`

Loads json data from disk, deserializes it and returns KvStore.

---

### Persistence (Version 3)

Data can be manually persisted to disk using JSON serialization:

**Saving:**

```rust
let mut store = KvStore::new("wal_path.wal");
store.set("key".to_string(), "value".to_string());
store.save_to_file("store.json")?;
```

**Loading:**

```rust
let store = KvStore::load_from_file("store.json", "wal_path.wal")?;
```

On `KvStore::new(wal_path)`, the WAL is replayed to restore any operations that occurred since the last snapshot

Ensures no data loss even if the program crashed before save_to_file was called

**Important:**

- Persistence is manual. Call `save_to_file()` before the store is dropped
- Data is serialized to JSON in pretty-print format (human-readable)
- Returns `Result<T, String>`, check for file I/O and serialization errors
- File paths are relative to the working directory where the program runs

**Example JSON output:**

```json
{
  "map": {
    "key": "value"
  }
}
```

---

### Thread Safety (Version 3)

`KvStore` is thread-safe: multiple threads can read and write concurrently.  
Wrap the store in an `Arc` to share ownership across threads:

```rust
use std::sync::Arc;
use std::thread;

let store = Arc::new(KvStore::new("wal_path.wal")?);
let store_clone = Arc::clone(&store);

thread::spawn(move || {
    store_clone.set("key".to_string(), "value".to_string());
}).join().unwrap();
```

---

### Limitations (Version 3)

- Persistence is manual; changes are not automatically saved.
- JSON files must be UTF-8 encoded.
- Not optimized for extremely large datasets; HashMap is stored entirely in memory.
- Multi-threaded performance is limited by RwLock contention for write-heavy workloads.

---

### Future Directions (Version 3)

Future iterations may include:

- Enhanced observability (logging, metrics)
- Automatic persistence (auto-save on changes)

Each feature will be added incrementally, maintaining simplicity and clarity.
