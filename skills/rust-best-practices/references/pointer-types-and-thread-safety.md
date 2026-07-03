# Pointer Types and Thread Safety

Rust encodes thread-safety through `Send` and `Sync`:

* `Send`: a value can be moved to another thread.
* `Sync`: shared references to the value can be used from multiple threads.

A pointer is thread-safe only when the data behind it satisfies the relevant
bounds.

## Pointer Selection Table

| Type | Ownership model | Thread-sharing guidance |
| --- | --- | --- |
| `&T` | Shared borrow | Thread-safe when `T: Sync`. |
| `&mut T` | Exclusive borrow | Movable when `T: Send`; still exclusive. |
| `Box<T>` | Single owner on heap | `Send`/`Sync` follows `T`. |
| `Rc<T>` | Non-atomic ref count | Single-thread only. |
| `Arc<T>` | Atomic ref count | Multi-thread shared ownership when `T: Send + Sync`. |
| `Cell<T>` | Copy-only interior mutability | Single-thread interior mutation; not `Sync`. |
| `RefCell<T>` | Runtime borrow checking | Single-thread shared mutation; not `Sync`; may panic. |
| `Mutex<T>` | Exclusive locked access | Multi-thread mutation when `T: Send`. |
| `RwLock<T>` | Shared reads or exclusive write | Multi-thread read-heavy mutation when `T: Send + Sync`. |
| `OnceCell<T>` | Single-thread one-time init | Use for local lazy initialization. |
| `LazyCell<T>` | Single-thread lazy init | Use when value construction can be delayed. |
| `OnceLock<T>` | Thread-safe one-time init | Use for global or shared initialization. |
| `LazyLock<T>` | Thread-safe lazy init | Use for global lazy initialization. |
| `*const T`, `*mut T` | Raw pointer | Unsafe; caller must uphold invariants. |

## Borrowing

Use `&T` for shared read access.

```rust
fn checksum(bytes: &[u8]) -> u8 {
    bytes.iter().copied().fold(0, u8::wrapping_add)
}

assert_eq!(checksum(&[1, 2, 3]), 6);
```

Use `&mut T` for exclusive mutation.

```rust
fn append_marker(buffer: &mut String, marker: &str) {
    buffer.push(':');
    buffer.push_str(marker);
}

let mut buffer = String::from("batch");
append_marker(&mut buffer, "ready");
assert_eq!(buffer, "batch:ready");
```

## `Box<T>`

Use `Box<T>` for heap allocation, recursive data, trait-object ownership, or large
values that should not move on the stack.

```rust
enum Expr {
    Value(i64),
    Add(Box<Expr>, Box<Expr>),
}
```

## `Rc<T>` and `Arc<T>`

Use `Rc<T>` for shared ownership within one thread. Use `Arc<T>` for shared
ownership across threads.

```rust
use std::sync::Arc;
use std::thread;

let table: Arc<[u16]> = Arc::from([10, 20, 30]);
let worker_table = Arc::clone(&table);

thread::spawn(move || assert_eq!(worker_table[1], 20))
    .join()
    .unwrap();
```

Use `Arc::clone(&value)` rather than `value.clone()` when you want to emphasize
shared ownership.

## Interior Mutability

Use `Cell<T>` for small `Copy` values in single-threaded code.

```rust
use std::cell::Cell;

let retries = Cell::new(0u8);
retries.set(retries.get() + 1);
assert_eq!(retries.get(), 1);
```

Use `RefCell<T>` only when compile-time borrowing cannot express the local design.
It enforces borrowing at runtime and can panic.

```rust
use std::cell::RefCell;

let notes = RefCell::new(Vec::new());
notes.borrow_mut().push("created");
assert_eq!(notes.borrow().len(), 1);
```

For multi-thread shared mutation, use `Mutex<T>` or `RwLock<T>`, usually behind
`Arc`.

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0usize));
let worker_counter = Arc::clone(&counter);

std::thread::spawn(move || {
    *worker_counter.lock().expect("counter lock poisoned") += 1;
})
.join()
.unwrap();

assert_eq!(*counter.lock().unwrap(), 1);
```

## One-Time Initialization

Use `OnceLock` or `LazyLock` for thread-safe initialization.

```rust
use std::collections::HashMap;
use std::sync::LazyLock;

static STATUS_CODES: LazyLock<HashMap<&'static str, u16>> = LazyLock::new(|| {
    HashMap::from([("queued", 1), ("running", 2), ("done", 3)])
});

assert_eq!(STATUS_CODES.get("done"), Some(&3));
```

Use `std::cell::OnceCell` or `LazyCell` for single-threaded local structures.

## Raw Pointers

Raw pointers are for FFI, low-level data structures, and performance-sensitive
code where safe references cannot model the operation. Keep unsafe blocks small
and document the invariants.

```rust
fn read_at(bytes: &[u8], index: usize) -> Option<u8> {
    if index >= bytes.len() {
        return None;
    }

    let ptr = bytes.as_ptr();
    // SAFETY: `index < bytes.len()` was checked above, so this read is in-bounds.
    Some(unsafe { *ptr.add(index) })
}
```
