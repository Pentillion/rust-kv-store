# Changelog

## [0.4.0] - 2026-01-25

### Added

- Write-Ahead Log (WAL) for durability
- All set, delete, and clear operations are logged to WAL before being applied to memory
- KvStore automatically replays WAL on startup to restore state after crashes
- WAL is truncated after snapshot (save_to_file) to prevent growth

## [0.3.0] - 2026-01-24

### Added

- Thread-safe access to `KvStore` using `RwLock`.
- `get`, `set`, `delete` and `clear` methods can now be safely called from multiple threads.
- Optional: shared ownership via `Arc` allows concurrent reads and writes across threads.

## [0.2.0] - 2026-01-17

### Added

- JSON serialization/deserialization for persistence via serde + serde_json
- `save_to_file()` method for saving store to disk
- `load_from_file()` method for loading store from disk

## [0.1.0] - Initial Release

### Added

- Basic in-memory key-value store
- Core API: get, set, delete, clear, len
