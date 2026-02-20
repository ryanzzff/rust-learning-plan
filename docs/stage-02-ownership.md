# Stage 2: Ownership & Borrowing (The Big One)

This is Rust's most important concept. Every other language either uses garbage collection (Go, Java, Python) or manual memory management (C, C++). Rust does neither — it uses **ownership rules checked at compile time**.

## The Three Ownership Rules

1. Each value in Rust has exactly **one owner**
2. When the owner goes out of scope, the value is **dropped** (freed)
3. Ownership can be **transferred** (moved), not duplicated (for heap data)

```rust
let s1 = String::from("hello");
let s2 = s1;          // s1's ownership MOVES to s2
// println!("{s1}");   // ERROR! s1 is no longer valid
println!("{s2}");      // OK — s2 owns the string now
```

## Mental Model: Stack vs Heap

```
Stack (fast, fixed-size)          Heap (flexible, slower)
┌──────────────┐                  ┌───────────────────┐
│ s2: ptr ─────│─────────────────►│ "hello"           │
│ len: 5       │                  └───────────────────┘
│ capacity: 5  │
└──────────────┘
```

When `s1` moves to `s2`, only the stack data (pointer, length, capacity) is copied. The heap data stays put. Rust invalidates `s1` to prevent a double-free.

## Move vs Copy

```rust
// integers COPY (they live on the stack — cheap to duplicate)
let x = 5;
let y = x;    // x is copied, not moved
println!("{x} {y}");  // both valid!

// Strings MOVE (they have heap data — expensive to duplicate)
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED
// s1 is now invalid
```

> **Rule of thumb**: Types that are small and live entirely on the stack implement the `Copy` trait (integers, floats, booleans, chars, tuples of Copy types). Everything else moves.

## References & Borrowing

Instead of transferring ownership, you can **borrow** a value:

```rust
fn calculate_length(s: &String) -> usize {  // s borrows the String
    s.len()
}   // s goes out of scope, but it doesn't own the String, so nothing is dropped

let s1 = String::from("hello");
let len = calculate_length(&s1);  // &s1 creates a reference
println!("{s1} has length {len}");  // s1 is still valid!
```

### Mutable References

```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
```

### The Borrowing Rules

1. You can have **either** one mutable reference **OR** any number of immutable references
2. References must always be valid (no dangling references)

```rust
let mut s = String::from("hello");

let r1 = &s;      // OK — immutable borrow
let r2 = &s;      // OK — multiple immutable borrows allowed
// let r3 = &mut s;  // ERROR! can't borrow mutably while immutable borrows exist

println!("{r1} {r2}");
// r1 and r2 are no longer used after this point (NLL: Non-Lexical Lifetimes)

let r3 = &mut s;   // OK now — r1 and r2 are done
```

> **Why this rule?** It prevents data races at compile time. A data race happens when two pointers access the same data and at least one is writing. The borrowing rules make this impossible.

## String vs &str

```rust
// String — owned, heap-allocated, growable
let mut s = String::from("hello");
s.push_str(" world");

// &str — borrowed string slice, immutable view into a string
let slice: &str = &s[0..5];   // "hello"
let literal: &str = "hello";  // string literals are &str (baked into binary)
```

```
String (owned)                     &str (borrowed slice)
┌──────────────┐                  ┌──────────────┐
│ ptr ─────────│──┐               │ ptr ─────────│──┐
│ len: 11      │  │               │ len: 5       │  │
│ capacity: 11 │  │               └──────────────┘  │
└──────────────┘  │                                  │
                  ▼                                  ▼
                  ┌─────────────────────┐
                  │ h e l l o   w o r l d │
                  └─────────────────────┘
```

> **Tip**: When writing functions, prefer `&str` as parameter type — it accepts both `&String` and `&str`.

## Lifetimes (Preview)

Lifetimes tell the compiler how long references are valid. You usually don't need to write them explicitly — the compiler infers them. You'll encounter explicit lifetimes when a function returns a reference and the compiler can't figure out which input it came from.

```rust
// The compiler needs to know: does the returned &str live as long as x or y?
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Don't worry about mastering lifetimes now — you'll build intuition as you go.

## Common Pitfalls

| Mistake | What the compiler says | Fix |
|---------|----------------------|-----|
| Using a value after it was moved | "value used after move" | Clone it, or use a reference |
| Two mutable borrows at once | "cannot borrow as mutable more than once" | Restructure to use one at a time |
| Returning a reference to local data | "returns a value referencing data owned by the current function" | Return the owned value instead |
| Mutating through an immutable reference | "cannot borrow as mutable" | Change to `&mut` |

## Exercise: String Manipulation Tool

Build a CLI tool that takes user input strings and performs operations (reverse, uppercase, count words, etc.) while demonstrating ownership transfers between functions.
