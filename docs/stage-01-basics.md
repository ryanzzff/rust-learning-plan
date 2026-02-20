# Stage 1: Hello Rust — Setup & Basics

## Setup

```bash
# Install Rust via rustup (the official installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Create your first project
cargo new guessing-game
cd guessing-game
cargo run   # compiles and runs — should print "Hello, world!"
```

## Key Concepts

### Cargo — Your Build System & Package Manager

Cargo does everything: build, test, manage dependencies, format code.

```bash
cargo new my-project    # create project (generates src/main.rs + Cargo.toml)
cargo build             # compile (debug mode, fast compile, slow binary)
cargo build --release   # compile (optimized, slow compile, fast binary)
cargo run               # build + run
cargo test              # run all tests
cargo check             # type-check without building (fastest feedback loop)
```

> **Insight**: Use `cargo check` while developing — it's much faster than `cargo build` because it skips code generation. Save `cargo build` for when you actually need to run.

### Variables & Mutability

```rust
let x = 5;         // immutable by default
// x = 6;          // ERROR! Can't mutate immutable variable

let mut y = 5;     // explicitly opt into mutability
y = 6;             // OK

let x = "hello";   // shadowing — creates a NEW variable named x
                    // the old x (= 5) is gone; this is NOT mutation
```

> **Why immutable by default?** The compiler can reason about code better when values don't change. When you see `let mut`, it signals "this value WILL change" — a useful visual cue.

### Type System

```rust
// Scalar types
let i: i32 = 42;           // signed 32-bit integer (default integer type)
let f: f64 = 3.14;         // 64-bit float (default float type)
let b: bool = true;
let c: char = 'z';         // 4 bytes — supports Unicode

// Compound types
let tup: (i32, f64, char) = (500, 6.4, 'y');
let (x, y, z) = tup;       // destructuring
let first = tup.0;          // access by index

let arr: [i32; 5] = [1, 2, 3, 4, 5];  // fixed-size array
let first = arr[0];
```

> **Insight**: Rust integers don't silently overflow in debug mode — the program panics. In release mode, they wrap around. This catches bugs early.

### Functions

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y   // no semicolon = this is the return value (an expression)
}

fn add_explicit(x: i32, y: i32) -> i32 {
    return x + y;   // explicit return also works, but is less idiomatic
}
```

> **Expression vs Statement**: In Rust, almost everything is an expression that returns a value. `x + y` is an expression. `x + y;` (with semicolon) is a statement that returns `()` (unit/nothing). This trips up beginners — if your function returns the wrong type, check for an accidental semicolon.

### Control Flow

```rust
// if/else — it's an expression! (can be assigned to a variable)
let number = if condition { 5 } else { 6 };

// loop — infinite loop with break value
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;   // break can return a value
    }
};

// while
while number != 0 {
    number -= 1;
}

// for — the idiomatic way to iterate
for element in [10, 20, 30] {
    println!("{element}");
}

for i in 0..5 {         // range: 0, 1, 2, 3, 4
    println!("{i}");
}

for i in (0..5).rev() {  // reverse: 4, 3, 2, 1, 0
    println!("{i}");
}
```

## Common Pitfalls

| Mistake | Fix |
|---------|-----|
| Forgetting `mut` on a variable you need to change | Add `let mut` |
| Adding a semicolon to the return expression | Remove the `;` on the last line |
| Using `==` to compare strings as `&str` vs `String` | Both work with `==`, but watch ownership |
| Array index out of bounds | Rust panics at runtime — no silent memory access |

## Exercise: Guessing Game

Build a CLI game where the program picks a random number (1-100) and the user guesses until they get it right. Uses: `rand` crate, `loop`, `match`, reading stdin, parsing strings to numbers.

```bash
# Add the rand crate to your project
cargo add rand
```
