# rust-kv-store

## Motivation

This project is meant to learn systems programming concepts in Rust, including:

- Ownership and memory management
- Correctness under failure
- Concurrency
- Persistent storage

---

## Current State

Currently, this is an in-memory key-value store implemented using a `HashMap`.
The API is defined and tests will be added to ensure correctness as the project evolves.

---

### Data Structure (Version 0)

Internally, the store is backed by a `HashMap<String, String>`:

```rust
use std::collections::HashMap;

struct KvStore {
    map: HashMap<String, String>,
}
```

---

### API (Version 0)

The in-memory key-value store exposes the following methods:

- `fn get(&self, key: &str) -> Option<String>`

  Returns the value associated with a key, or `None` if the key does not exist.

- `fn set(&mut self, key: String, value: String)`

  Sets the value associated with a key. Overwrites any existing value.

- `fn delete(&mut self, key: &str) -> Option<String>`

  Deletes the value associated with a key. Returns the deleted value if it existed.

---

### Future Directions

Future iterations may include:

- Persistence to disk
- Concurrency and thread-safe access
- Enhanced observability (logging, metrics)

Each feature will be added incrementally, maintaining simplicity and clarity.
