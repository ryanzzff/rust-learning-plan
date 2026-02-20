# Stage 10: Capstone Project

## Purpose

Combine everything from stages 1-9 into a real-world project. Pick one:

### Option A: CLI Tool with `clap`

Build a markdown link checker that scans `.md` files and verifies URLs.

**Concepts used**: File I/O, string parsing, error handling, concurrency (parallel HTTP checks), project structure, testing.

```bash
cargo add clap --features derive
cargo add reqwest --features blocking   # HTTP client
cargo add regex
```

### Option B: HTTP Server with `axum`

Build a REST API (e.g., bookmarks manager) with JSON endpoints.

**Concepts used**: Structs/enums for models, traits for shared behavior, error handling, modules, testing, shared state with Arc/Mutex.

```bash
cargo add axum
cargo add tokio --features full         # async runtime
cargo add serde --features derive       # JSON serialization
cargo add serde_json
```

### Option C: Database CRUD App with `sqlx`

Build a task manager backed by SQLite.

**Concepts used**: All the above, plus database interaction, migrations, async.

```bash
cargo add sqlx --features "runtime-tokio sqlite"
cargo add tokio --features full
```

## Project Checklist

Use this to verify you've applied learnings from each stage:

- [ ] **Stage 1**: Project created with Cargo, uses variables and control flow
- [ ] **Stage 2**: Ownership is handled correctly — no unnecessary `.clone()`
- [ ] **Stage 3**: Data modeled with structs and enums, uses pattern matching
- [ ] **Stage 4**: Errors handled with `Result` and `?`, no stray `unwrap()`
- [ ] **Stage 5**: Uses iterators and closures for data processing
- [ ] **Stage 6**: At least one trait defined and implemented
- [ ] **Stage 7**: Code organized into modules with proper visibility
- [ ] **Stage 8**: Has unit tests and integration tests
- [ ] **Stage 9**: Uses concurrency (threads, async, or channels)
- [ ] **Stage 10**: Runs as a complete, usable application

## Beyond the Basics

After completing the capstone, explore:

| Topic | Crate/Resource |
|-------|---------------|
| Async/Await | `tokio`, `async-std` |
| Serialization | `serde` (JSON, TOML, YAML) |
| CLI frameworks | `clap`, `dialoguer`, `indicatif` |
| Web frameworks | `axum`, `actix-web`, `rocket` |
| Database | `sqlx`, `diesel`, `sea-orm` |
| Error handling | `thiserror` (libraries), `anyhow` (applications) |
| Logging | `tracing`, `env_logger` |
| HTTP client | `reqwest` |
| Testing | `mockall`, `proptest`, `criterion` (benchmarks) |
| FFI | Calling C from Rust, or Rust from other languages |
| Macros | Declarative (`macro_rules!`) and procedural macros |
| Unsafe Rust | Raw pointers, FFI, when the compiler can't prove safety |
