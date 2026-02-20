# Stage 6: Traits & Generics

## Traits — Shared Behavior

Traits are like interfaces — they define behavior that types can implement.

```rust
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation — types can override or use as-is
    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..20])
    }
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, &self.content[..50])
    }
    // preview() uses the default implementation
}
```

## Generics — Write Once, Use for Many Types

```rust
// Without generics — need separate functions for each type
fn largest_i32(list: &[i32]) -> &i32 { /* ... */ }
fn largest_f64(list: &[f64]) -> &f64 { /* ... */ }

// With generics — one function, any type that supports comparison
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

> **Insight**: Generics in Rust have zero runtime cost. The compiler generates specialized code for each concrete type used (monomorphization). `largest::<i32>` and `largest::<f64>` become two separate functions in the binary.

## Trait Bounds

Trait bounds constrain what a generic type can do:

```rust
// Syntax 1: Inline bound
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Syntax 2: impl Trait (simpler for single params)
fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Syntax 3: Where clause (cleaner for complex bounds)
fn complex_function<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: Summary + std::fmt::Debug,
{
    format!("{} and {:?}", t.summarize(), u)
}
```

### Multiple Bounds with `+`

```rust
fn process<T: Summary + Clone + Debug>(item: T) {
    // T must implement Summary AND Clone AND Debug
}
```

## Return `impl Trait`

```rust
fn make_summarizable() -> impl Summary {
    Article {
        title: String::from("Breaking"),
        content: String::from("Something happened that is noteworthy..."),
    }
}
// Caller only knows it returns "something that implements Summary"
```

> **Limitation**: `impl Trait` in return position can only return one concrete type. You can't conditionally return different types. For that, use trait objects (`Box<dyn Trait>`).

## Trait Objects — Dynamic Dispatch

```rust
// Static dispatch (generics) — compiler knows the type at compile time
fn notify(item: &impl Summary) { }  // monomorphized per type

// Dynamic dispatch (trait objects) — type determined at runtime
fn notify(item: &dyn Summary) { }   // uses vtable, slight overhead

// Storing mixed types in a collection
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(article),
    Box::new(tweet),
];
```

> **When to use which**: Use generics (static dispatch) by default — it's faster. Use `dyn Trait` when you need to store different types in the same collection or return different types from a function.

## Common Derive Traits

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
```

| Trait | What it gives you |
|-------|-------------------|
| `Debug` | `{:?}` formatting for printing |
| `Clone` | `.clone()` method for explicit duplication |
| `Copy` | Implicit copying (only for simple stack-only types) |
| `PartialEq` / `Eq` | `==` and `!=` comparison |
| `Hash` | Usable as HashMap/HashSet key |
| `Default` | `Type::default()` for zero/empty values |
| `PartialOrd` / `Ord` | `<`, `>`, `<=`, `>=` comparison and sorting |

## Common Pitfalls

| Mistake | Fix |
|---------|-----|
| "the trait bound is not satisfied" | Add the required trait bound to your generic parameter |
| Trying to return different types with `impl Trait` | Use `Box<dyn Trait>` instead |
| Deriving `Copy` on a struct with `String` fields | `String` isn't `Copy` — use `Clone` and call `.clone()` explicitly |
| Trait method conflicts (same method name from two traits) | Use fully qualified syntax: `<Type as Trait>::method()` |

## Exercise: Plugin System

Build a plugin system where different "plugins" implement a trait and are loaded into a runner that executes them. Demonstrates trait objects, generics, and dynamic dispatch.
