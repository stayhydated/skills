# Generics, Static Dispatch, and Dynamic Dispatch

> Static where you can, dynamic where you must.

Rust supports polymorphism through generics, `impl Trait`, and trait objects. The
right choice depends on whether the concrete type is known at compile time, needs
to relate to other types, or must be erased at runtime.

## Generics

Use named generics when relationships matter: two arguments must have the same
concrete type, a return type depends on an input type, or bounds are easier to
read in a `where` clause.

```rust
trait Encode {
    fn encode(&self, out: &mut Vec<u8>);
}

fn encode_all<I>(items: I) -> Vec<u8>
where
    I: IntoIterator,
    I::Item: Encode,
{
    let mut out = Vec::new();
    for item in items {
        item.encode(&mut out);
    }
    out
}
```

Use `where` clauses when bounds get longer than a single simple trait.

```rust
fn merge_sorted<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: Clone + Ord,
{
    left.iter().chain(right).cloned().collect()
}
```

## `impl Trait` for Input Ergonomics

Use argument-position `impl Trait` when the concrete type does not need a name.

```rust
fn emit_line(mut write: impl std::io::Write, line: &str) -> std::io::Result<()> {
    writeln!(write, "{line}")
}
```

Do not use `impl Trait` when two arguments must be the same concrete type. Use a
named generic instead.

```rust
fn same_codec<C>(left: C, right: C) -> (C, C)
where
    C: Clone,
{
    (left.clone(), right)
}
```

## Return-Position `impl Trait`

Return `impl Trait` to hide a concrete iterator or future while preserving static
dispatch.

```rust
fn non_empty_segments(input: &str) -> impl Iterator<Item = &str> {
    input.split('/').filter(|segment| !segment.is_empty())
}

let segments: Vec<_> = non_empty_segments("/alpha//beta/").collect();
assert_eq!(segments, ["alpha", "beta"]);
```

For public traits, be careful with return-position `impl Trait`: the hidden type
is still part of the compiler-checked API surface. Do not expose private hidden
return types in ways that conflict with visibility or object-safety requirements.

## Static Dispatch

Generic and `impl Trait` code is statically dispatched and monomorphized. This
usually enables inlining and removes runtime dispatch, but it may increase compile
time and binary size.

```rust
trait Weight {
    fn weight(&self) -> u64;
}

fn total_weight<T>(items: &[T]) -> u64
where
    T: Weight,
{
    items.iter().map(Weight::weight).sum()
}
```

Prefer static dispatch for hot paths, library helpers where callers know the
concrete type, and code that does not need runtime heterogeneity.

## Dynamic Dispatch with `dyn Trait`

Use trait objects when you need heterogeneous values behind one interface or a
runtime plugin boundary.

```rust
struct Batch {
    lines: Vec<String>,
}

trait Stage {
    fn run(&self, batch: &mut Batch);
}

struct TrimStage;
impl Stage for TrimStage {
    fn run(&self, batch: &mut Batch) {
        for line in &mut batch.lines {
            *line = line.trim().to_owned();
        }
    }
}

struct LowercaseStage;
impl Stage for LowercaseStage {
    fn run(&self, batch: &mut Batch) {
        for line in &mut batch.lines {
            *line = line.to_ascii_lowercase();
        }
    }
}

fn run_pipeline(stages: &[Box<dyn Stage>], batch: &mut Batch) {
    for stage in stages {
        stage.run(batch);
    }
}
```

Prefer `&dyn Trait` when you do not need ownership, `Box<dyn Trait>` for owned
heterogeneous values, and `Arc<dyn Trait + Send + Sync>` for shared objects across
threads.

## Dyn Compatibility Checklist

A trait must be dyn-compatible before it can be used as `dyn Trait`. Keep dynamic
traits simple:

* Methods should take `&self`, `&mut self`, or an explicitly boxed/owned receiver.
* Avoid generic methods on dynamic traits.
* Avoid methods that return bare `Self` unless they are restricted with
  `where Self: Sized`.
* Keep associated constants and complex generic associated types out of trait
  objects unless the dyn-compatibility rules allow the exact shape.

```rust
trait Runnable {
    fn run(&self);
}

trait Factory {
    fn create<T>() -> T;
}
```

`Runnable` can be a trait object. `Factory` cannot because `create` is generic.

## Trade-Off Summary

| Choice | Strength | Cost |
| --- | --- | --- |
| Named generics | Express type relationships clearly | More syntax |
| `impl Trait` arguments | Concise static dispatch | Cannot name or relate hidden type |
| Return `impl Trait` | Hides concrete iterator/future | Hidden type still affects public API |
| `dyn Trait` | Runtime heterogeneity and stable boundary | Vtable dispatch, object-safety limits |

Start static. Introduce dynamic dispatch when type erasure is part of the design,
not just to avoid generics.
