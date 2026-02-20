# Stage 5: Collections & Iterators

## Collections

### Vec\<T\> — Growable Array

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);

let v = vec![1, 2, 3];  // macro shorthand

let third = &v[2];          // panics if out of bounds
let third = v.get(2);       // returns Option<&i32> — safe
```

### HashMap\<K, V\>

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Entry API — insert only if key doesn't exist
scores.entry(String::from("Blue")).or_insert(0);

// Update based on old value
let count = scores.entry(String::from("Blue")).or_insert(0);
*count += 1;   // dereference to modify
```

> **Insight**: The entry API is idiomatic Rust for "get or insert." It avoids the double lookup that checking `.contains_key()` then `.insert()` would require.

### HashSet\<T\>

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert("apple");
set.insert("banana");
set.insert("apple");  // duplicate — ignored

println!("{}", set.len());  // 2
println!("{}", set.contains("apple"));  // true
```

## Iterators

Iterators are Rust's approach to functional-style data processing. They're lazy — nothing happens until you consume them.

```rust
let v = vec![1, 2, 3, 4, 5];

// Iterator adapters (lazy — build a pipeline)
let result: Vec<i32> = v.iter()
    .filter(|&&x| x > 2)       // keep only > 2
    .map(|&x| x * 10)          // multiply each by 10
    .collect();                  // consume and build Vec

// result = [30, 40, 50]
```

### Three Ways to Iterate

```rust
let v = vec![String::from("a"), String::from("b")];

// Borrow immutably — v is still usable after
for s in &v { }        // s is &String
for s in v.iter() { }  // equivalent

// Borrow mutably — can modify elements
for s in &mut v { }         // s is &mut String
for s in v.iter_mut() { }   // equivalent

// Take ownership — v is consumed (moved)
for s in v { }              // s is String, v is gone
for s in v.into_iter() { }  // equivalent
```

> **Insight**: `.iter()` vs `.into_iter()` is an ownership question. Use `.iter()` when you want to keep the collection, `.into_iter()` when you're done with it.

## Closures

Closures are anonymous functions that capture their environment.

```rust
let threshold = 10;

// Closure captures `threshold` from the environment
let is_above = |x: &i32| *x > threshold;

let nums = vec![5, 15, 25, 3, 12];
let above: Vec<&i32> = nums.iter().filter(|x| is_above(x)).collect();
```

### Closure Capture Modes

```rust
let name = String::from("Ryan");

let greet = || println!("Hello, {name}");   // borrows name (&name)
greet();
println!("{name}");  // still valid — closure only borrowed

let consume = move || println!("Bye, {name}");  // MOVES name into closure
consume();
// println!("{name}");  // ERROR — name was moved into the closure
```

> **`move` keyword**: Forces the closure to take ownership of captured variables. Essential when passing closures to threads (stage 9) — the thread might outlive the scope.

## Common Iterator Methods

| Method | Description | Example |
|--------|-------------|---------|
| `map` | Transform each element | `.map(\|x\| x * 2)` |
| `filter` | Keep elements matching predicate | `.filter(\|x\| *x > 0)` |
| `fold` | Accumulate into single value | `.fold(0, \|acc, x\| acc + x)` |
| `collect` | Build a collection from iterator | `.collect::<Vec<_>>()` |
| `enumerate` | Add index to each element | `.enumerate()` → `(0, val), (1, val)...` |
| `zip` | Pair elements from two iterators | `a.iter().zip(b.iter())` |
| `find` | First element matching predicate | `.find(\|x\| *x > 5)` → `Option` |
| `any` / `all` | Boolean check | `.any(\|x\| *x > 5)` → `bool` |
| `sum` | Sum all elements | `.sum::<i32>()` |
| `count` | Count elements | `.count()` |
| `flatten` | Flatten nested iterators | `vec![vec![1,2], vec![3]]` → `[1,2,3]` |

## Common Pitfalls

| Mistake | Fix |
|---------|-----|
| Forgetting `.collect()` — "nothing happens" | Iterators are lazy; add `.collect()` or `for` loop to consume |
| Wrong reference depth in closures (`&&x` vs `&x`) | `.iter()` gives `&T`, `.filter()` gives `&&T` — destructure with pattern |
| Type inference fails on `.collect()` | Add type annotation: `.collect::<Vec<_>>()` |
| Modifying a collection while iterating | Build a new collection with `.filter().collect()` instead |

## Exercise: Word Frequency Counter

Build a CLI tool that reads a text file and outputs word frequencies, sorted by count. Uses: HashMap, iterators, closures, file I/O.
