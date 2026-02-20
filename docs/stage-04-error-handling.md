# Stage 4: Error Handling

Rust has no exceptions. Errors are values — you return them, match on them, and propagate them with `?`.

## Result\<T, E\>

```rust
enum Result<T, E> {
    Ok(T),    // success with value T
    Err(E),   // failure with error E
}

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()   // returns Result
}

match parse_number("42") {
    Ok(n) => println!("Parsed: {n}"),
    Err(e) => println!("Failed: {e}"),
}
```

## The `?` Operator — Error Propagation

The `?` operator is Rust's way to "bubble up" errors — like `throw` but explicit.

```rust
fn read_username() -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open("username.txt")?;  // returns Err early if fails
    let mut username = String::new();
    file.read_to_string(&mut username)?;                    // same here
    Ok(username)
}
```

> **What `?` does**: If the Result is `Ok(v)`, it unwraps to `v`. If it's `Err(e)`, it returns `Err(e)` from the current function immediately. It's syntactic sugar for a match statement.

Without `?`, the same code would be:

```rust
fn read_username() -> Result<String, std::io::Error> {
    let file_result = std::fs::File::open("username.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    // ... more matching ...
}
```

## Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseError(String),
    IoError(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(name) => write!(f, "Not found: {name}"),
            AppError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            AppError::IoError(err) => write!(f, "IO error: {err}"),
        }
    }
}

// Enable ? to convert io::Error into AppError automatically
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}
```

> **Shortcut**: The `thiserror` crate generates these impls for you with derive macros. For applications (not libraries), the `anyhow` crate provides a catch-all error type.

## When to Use What

| Situation | Use |
|-----------|-----|
| Prototype / script | `unwrap()` or `anyhow::Result` |
| Library code | Custom error enum or `thiserror` |
| Unrecoverable bug | `panic!` (e.g., index out of bounds) |
| Expected failure | `Result<T, E>` |
| Value might be absent (not an error) | `Option<T>` |

## `unwrap()` and `expect()`

```rust
// unwrap: panics with a generic message on Err/None
let value = some_result.unwrap();

// expect: panics with YOUR message on Err/None
let value = some_result.expect("config file should exist");
```

> **Rule**: Never use `unwrap()` in production code. Use `expect()` if you're truly sure it can't fail (and explain why in the message). Otherwise, handle the error properly.

## Common Patterns

```rust
// Convert Option to Result
let value = some_option.ok_or(AppError::NotFound("item".into()))?;

// Map errors to a different type
let content = std::fs::read_to_string("file.txt")
    .map_err(|e| AppError::IoError(e))?;

// Provide default on error
let port = env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse::<u16>()
    .unwrap_or(8080);
```

## Common Pitfalls

| Mistake | Fix |
|---------|-----|
| Using `unwrap()` everywhere | Replace with `?` or proper matching |
| `?` in `main()` won't compile | Change main signature to `fn main() -> Result<(), Box<dyn Error>>` |
| Mixing error types with `?` | Implement `From` for your error type, or use `anyhow` |
| Ignoring errors with `let _ = ...` | At minimum, log them |

## Exercise: CSV File Parser

Build a tool that reads a CSV file, parses rows into structs, and handles all failure modes gracefully (missing file, bad format, wrong types).
