# Stage 3: Structs, Enums & Pattern Matching

## Structs

Structs group related data together — similar to classes in other languages, but without inheritance.

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

let user1 = User {
    username: String::from("ryan"),
    email: String::from("ryan@example.com"),
    active: true,
    sign_in_count: 1,
};

// Struct update syntax — copy remaining fields from another instance
let user2 = User {
    email: String::from("ryan2@example.com"),
    ..user1   // moves String fields from user1!
};
```

> **Gotcha**: The `..user1` syntax moves owned fields (like `String`). After this, `user1.username` is invalid (moved to `user2`), but `user1.active` and `user1.sign_in_count` are still valid because they implement `Copy`.

### Methods (impl blocks)

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function (like a static method) — no &self
    fn square(size: f64) -> Self {
        Self { width: size, height: size }
    }

    // Method — takes &self (borrows the instance)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Mutable method — can modify the instance
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

let mut rect = Rectangle::square(5.0);  // associated function call
println!("Area: {}", rect.area());       // method call
rect.scale(2.0);
```

> **`&self` vs `&mut self` vs `self`**: This is ownership applied to methods. `&self` borrows immutably, `&mut self` borrows mutably, `self` takes ownership (consuming the struct — rare, used for transformations).

## Enums

Enums in Rust are far more powerful than in most languages — each variant can hold different data.

```rust
enum Message {
    Quit,                        // no data
    Move { x: i32, y: i32 },    // named fields (like a struct)
    Write(String),               // single value
    ChangeColor(i32, i32, i32),  // tuple-like
}

let msg = Message::Write(String::from("hello"));
```

> **Insight**: Rust enums are "algebraic data types" (tagged unions). They're one of Rust's most powerful features — they let you model "this value is one of these possible variants" with type safety.

## Pattern Matching with `match`

```rust
fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({x}, {y})"),
        Message::Write(text) => println!("Writing: {text}"),
        Message::ChangeColor(r, g, b) => println!("Color: ({r}, {g}, {b})"),
    }
}
```

`match` is **exhaustive** — you must handle every variant. The compiler enforces this, preventing forgotten cases.

```rust
match value {
    1 => println!("one"),
    2 | 3 => println!("two or three"),   // multiple patterns
    4..=10 => println!("four to ten"),    // range pattern
    _ => println!("something else"),      // catch-all
}
```

## Option\<T\> — Rust's Null Replacement

Rust has no `null`. Instead, it uses `Option<T>`:

```rust
enum Option<T> {
    Some(T),   // there's a value
    None,      // there's no value
}

let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;

// You MUST handle the None case — the compiler forces it
match some_number {
    Some(n) => println!("Got: {n}"),
    None => println!("No value"),
}
```

### `if let` — Shorthand for Single-Pattern Matches

```rust
// Instead of this:
match some_number {
    Some(n) => println!("{n}"),
    None => {},
}

// Write this:
if let Some(n) = some_number {
    println!("{n}");
}
```

> **Why no null?** Tony Hoare (inventor of null) called it his "billion-dollar mistake." With `Option<T>`, the type system tracks whether a value might be absent. You can never accidentally use a None value — the compiler won't let you.

## Common Patterns

```rust
// Unwrap with default
let value = some_option.unwrap_or(0);

// Transform the inner value
let doubled = some_option.map(|n| n * 2);

// Chain operations
let result = some_option
    .filter(|n| *n > 0)
    .map(|n| n.to_string());
```

## Common Pitfalls

| Mistake | Fix |
|---------|-----|
| Forgetting a `match` arm | Add the missing variant or use `_` catch-all |
| Moving a field out of a struct unintentionally with `..` | Use `.clone()` if you need both |
| Using `unwrap()` on `None` | Panics at runtime — use `match` or `unwrap_or` |
| Trying to access enum variant data without matching | Use `match` or `if let` to destructure |

## Exercise: Task Tracker (Todo CLI)

Build a terminal todo app with commands: add, complete, list, remove. Model tasks with a struct and status with an enum.
