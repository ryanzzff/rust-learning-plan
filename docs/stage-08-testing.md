# Stage 8: Testing

## Unit Tests

Unit tests live in the same file as the code they test, inside a `#[cfg(test)]` module.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]                // only compiled when running tests
mod tests {
    use super::*;           // import everything from the parent module

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }
}
```

```bash
cargo test                      # run all tests
cargo test test_add             # run tests matching name
cargo test -- --show-output     # show println! output from passing tests
cargo test -- --test-threads=1  # run tests sequentially (useful for shared state)
```

> **Insight**: `#[cfg(test)]` means the test module is completely removed from release builds — zero overhead. The tests can access private functions through `use super::*`, which is intentional and idiomatic.

## Assertion Macros

```rust
assert!(condition);                          // fails if false
assert_eq!(left, right);                     // fails if left != right (shows both values)
assert_ne!(left, right);                     // fails if left == right
assert!(value > 0, "value was {}", value);   // custom error message
```

## Testing Errors

```rust
#[test]
#[should_panic]
fn test_panics() {
    panic!("oh no");
}

#[test]
#[should_panic(expected = "out of bounds")]   // must contain this substring
fn test_specific_panic() {
    let v = vec![1, 2, 3];
    v[99];  // panics with "index out of bounds"
}

// Testing Result-returning functions
#[test]
fn test_result() -> Result<(), String> {
    let result = "42".parse::<i32>().map_err(|e| e.to_string())?;
    assert_eq!(result, 42);
    Ok(())
}
```

## Integration Tests

Integration tests live in the `tests/` directory and can only access your crate's public API.

```
my-project/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs    # each file is a separate crate
```

```rust
// tests/integration_test.rs
use my_project;   // import your crate as an external dependency

#[test]
fn test_public_api() {
    assert_eq!(my_project::add(2, 3), 5);
}
```

> **Key difference**: Integration tests are separate crates — they can only call `pub` functions. This tests your API from a user's perspective. If a test needs access to internal details, it should be a unit test.

## Documentation Tests

```rust
/// Doubles a number.
///
/// ```
/// assert_eq!(my_crate::double(5), 10);
/// ```
pub fn double(x: i32) -> i32 {
    x * 2
}
```

```bash
cargo test --doc    # run only doc tests
```

## Test Organization Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Group related tests with nested modules
    mod add_tests {
        use super::*;

        #[test]
        fn positive_numbers() { assert_eq!(add(1, 2), 3); }

        #[test]
        fn negative_numbers() { assert_eq!(add(-1, -2), -3); }
    }

    // Test helper functions (not marked with #[test])
    fn setup() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    #[test]
    fn test_with_setup() {
        let data = setup();
        assert_eq!(data.len(), 5);
    }
}
```

## Common Pitfalls

| Mistake | Fix |
|---------|-----|
| `assert_eq!` with types that don't implement `Debug`/`PartialEq` | Derive both: `#[derive(Debug, PartialEq)]` |
| Tests pass individually but fail together | Tests run in parallel by default — avoid shared mutable state |
| Integration test can't find your crate | You need `src/lib.rs` (not just `main.rs`) for integration tests |
| Doc test won't compile | Make sure to `use` the right items; doc tests are standalone |

## Exercise: Add Tests to Previous Projects

Go back to stages 1-7 and add comprehensive unit tests and integration tests. Practice writing tests that document expected behavior.
