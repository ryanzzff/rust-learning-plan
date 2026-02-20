# Stage 7: Modules, Crates & Project Structure

## Module System

Modules organize code within a crate. Think of them as namespaces with visibility control.

```rust
// In src/main.rs or src/lib.rs

mod garden {           // declare module inline
    pub mod vegetables {
        pub fn plant() {
            println!("Planting!");
        }
    }

    mod secrets {       // private module — only accessible within `garden`
        fn water() {}
    }
}

use garden::vegetables;    // bring into scope
vegetables::plant();
```

> **Visibility rule**: Everything is private by default. `pub` makes it public to the parent module. This is different from languages where `public` means "visible to everyone" — in Rust, `pub` means "visible to whoever can see the parent."

## File-Based Modules

```
src/
├── main.rs           # crate root
├── garden.rs         # or garden/mod.rs — the garden module
└── garden/
    └── vegetables.rs # garden::vegetables submodule
```

```rust
// src/main.rs
mod garden;           // loads from src/garden.rs or src/garden/mod.rs
use garden::vegetables;

fn main() {
    vegetables::plant();
}
```

```rust
// src/garden.rs
pub mod vegetables;   // loads from src/garden/vegetables.rs
```

```rust
// src/garden/vegetables.rs
pub fn plant() {
    println!("Planting!");
}
```

> **Two styles**: `garden.rs` (modern, recommended) vs `garden/mod.rs` (older style). Both work; don't mix them in the same project.

## Crate vs Package

```
my-package/              # Package (has Cargo.toml)
├── Cargo.toml
├── src/
│   ├── main.rs          # Binary crate root
│   └── lib.rs           # Library crate root (optional)
└── tests/               # Integration tests
    └── integration.rs
```

| Term | Meaning |
|------|---------|
| **Package** | A `Cargo.toml` + one or more crates |
| **Crate** | A compilation unit (binary or library) |
| **Module** | Code organization within a crate |

## Using External Crates

```bash
cargo add serde --features derive    # add crate with feature flag
cargo add tokio -F full              # shorthand for --features
```

```toml
# Cargo.toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
rand = "0.8"
```

```rust
use serde::{Serialize, Deserialize};
use rand::Rng;
```

## Documentation Comments

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// Panics if the result overflows `i32`.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```bash
cargo doc --open    # generates HTML docs and opens in browser
cargo test          # runs doc examples as tests!
```

> **Insight**: The `///` examples are compiled and executed by `cargo test`. This means your docs can never go stale — if the example breaks, the test fails. This is one of Rust's best features for maintaining documentation.

## Recommended Project Layout

```
my-project/
├── Cargo.toml
├── src/
│   ├── main.rs          # thin — just calls lib
│   ├── lib.rs           # public API, re-exports
│   ├── config.rs        # configuration module
│   ├── error.rs         # error types
│   └── models/
│       ├── mod.rs
│       ├── user.rs
│       └── task.rs
└── tests/
    └── integration.rs   # integration tests (can only use public API)
```

## Common Pitfalls

| Mistake | Fix |
|---------|-----|
| Can't access a function from another module | Mark it `pub` |
| "unresolved import" | Add `mod module_name;` in the parent |
| Circular module dependencies | Restructure — move shared types to a common module |
| `use super::*` confusion | `super` refers to the parent module, `crate` to the root |

## Exercise: Refactor Into Multi-Module Project

Take your previous exercises and reorganize them into a well-structured project with separate modules, proper visibility, and documentation comments.
