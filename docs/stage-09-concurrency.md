# Stage 9: Concurrency

Rust's ownership system prevents data races at compile time — this is called "fearless concurrency."

## Threads

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("spawned thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("main thread: {i}");
    thread::sleep(Duration::from_millis(1));
}

handle.join().unwrap();  // wait for spawned thread to finish
```

### Moving Data Into Threads

```rust
let name = String::from("Ryan");

// ERROR: closure may outlive the current function, but it borrows `name`
// let handle = thread::spawn(|| println!("{name}"));

// FIX: use `move` to transfer ownership to the thread
let handle = thread::spawn(move || {
    println!("Hello, {name}");
});
// name is no longer accessible here — it was moved
```

> **Why `move` is required**: The spawned thread might live longer than the scope that created it. Borrowing would create a dangling reference. `move` transfers ownership, so the thread owns the data and can safely use it.

## Message Passing (Channels)

Channels implement "share memory by communicating" (Go-style).

```rust
use std::sync::mpsc;   // multi-producer, single-consumer

let (tx, rx) = mpsc::channel();

// Clone sender for multiple producers
let tx2 = tx.clone();

thread::spawn(move || {
    tx.send(String::from("hello from thread 1")).unwrap();
});

thread::spawn(move || {
    tx2.send(String::from("hello from thread 2")).unwrap();
});

// Receive — blocks until a message arrives
for received in rx {
    println!("Got: {received}");
}
```

> **Insight**: `send()` takes ownership of the value. Once you send data through a channel, you can no longer use it in the sending thread. This prevents shared mutable state by design.

## Shared State (Mutex & Arc)

When you need multiple threads to access the same data:

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));   // Arc = atomic reference counting
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);  // clone the Arc (not the Mutex)
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();  // acquire lock
        *num += 1;
        // lock is automatically released when `num` goes out of scope
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());  // 10
```

### Why Arc + Mutex?

```
                    Arc (thread-safe reference counting)
                    ┌──────────┐
Thread 1 ──────────►│          │
Thread 2 ──────────►│  Mutex   │──── protects ────► data
Thread 3 ──────────►│          │
                    └──────────┘

Arc: allows multiple owners across threads (like Rc but thread-safe)
Mutex: ensures only one thread accesses the data at a time
```

> **`Rc` vs `Arc`**: `Rc<T>` (Reference Counted) is for single-threaded shared ownership. `Arc<T>` (Atomically Reference Counted) is the thread-safe version. Using `Rc` in a multi-threaded context won't compile — the compiler catches it.

## Send and Sync Traits

These marker traits determine what can cross thread boundaries:

| Trait | Meaning |
|-------|---------|
| `Send` | Type can be transferred to another thread |
| `Sync` | Type can be shared between threads via references |

Most types are `Send + Sync` automatically. Notable exceptions:
- `Rc<T>` — not `Send` (use `Arc<T>` instead)
- `Cell<T>` / `RefCell<T>` — not `Sync` (use `Mutex<T>` instead)
- Raw pointers — neither

> **Insight**: You rarely implement these traits yourself. The compiler derives them automatically. When it refuses to let you share data across threads, it's telling you to use thread-safe alternatives.

## Common Pitfalls

| Mistake | Fix |
|---------|-----|
| Using `Rc` across threads | Switch to `Arc` |
| Forgetting `move` on thread closure | Add `move` to transfer ownership |
| Deadlock from locking two mutexes | Always lock in the same order; keep critical sections short |
| Calling `.lock().unwrap()` — what if it panics? | A poisoned mutex (from a panicked thread) — `unwrap()` is usually fine here |

## Exercise: Parallel File Search

Build a tool that searches for a string pattern across multiple files in parallel (like a simplified `grep -r`). Uses: threads, channels, Arc/Mutex, file I/O.
