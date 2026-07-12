# Type State Pattern

## Contents

* [When type-state fits](#when-type-state-fits)
* [Statum mental model](#statum-mental-model)
* [Lifecycle example](#example-upload-session)
* [Rehydrating state](#rehydrating-persisted-state)
* [Builder boundary](#type-state-is-not-a-builder)
* [When to avoid type-state](#avoid-type-state-when)
* [Cautions](#cautions)

The type-state pattern models lifecycle state at compile time. Instead of storing
a runtime flag and checking it before every operation, represent each meaningful
state as a distinct type and expose only the methods valid for that state.

> Invalid operations become compile errors instead of runtime bugs.

In this skill, use [`statum`](https://docs.rs/statum/latest/statum/) for
production type-state code. Do not hand-roll `PhantomData`, marker structs, or
custom generic state machinery unless the user explicitly asks for the manual
pattern.

## When Type-State Fits

Use type-state when state changes which operations are legal:

* A transfer must be prepared before it can be committed.
* A session must be authenticated before it can send privileged commands.
* A buffer must be filled before it can be sealed.
* Persisted workflow state must be validated once before internal code treats it
  as trusted.

Use a plain enum and `match` for local branching. Use `bon` for construction. Use
Statum when the value has a real typed lifecycle.

## Statum Mental Model

Statum provides a macro-based type-state surface:

* `#[state]` defines the legal phases.
* `#[machine]` defines shared context carried across phases.
* `#[transition]` defines legal state transitions.
* `#[validators]` rebuilds typed machines from persisted or external data.

## Example: Upload Session

```rust
use statum::{machine, state, transition};

#[state]
enum UploadState {
    Empty,
    Buffered(BufferInfo),
    Stored,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BufferInfo {
    bytes: usize,
}

#[machine]
struct UploadSession<UploadState> {
    id: String,
}

#[transition]
impl UploadSession<Empty> {
    fn buffer(self, bytes: usize) -> UploadSession<Buffered> {
        self.transition_with(BufferInfo { bytes })
    }
}

#[transition]
impl UploadSession<Buffered> {
    fn store(self) -> UploadSession<Stored> {
        self.transition()
    }
}

let session = UploadSession::<Empty>::builder()
    .id("upload-1".to_owned())
    .build();

let buffered = session.buffer(4096);
assert_eq!(buffered.state_data.bytes, 4096);

let _stored = buffered.store();
```

After buffering, methods defined only on `UploadSession<Empty>` are gone. After
storing, methods defined only on `UploadSession<Buffered>` are gone. The method
surface follows the lifecycle.

## Rehydrating Persisted State

Compile-time state does not remove runtime validation at system boundaries. A row,
event, or payload is still untrusted. Use validators to rebuild exactly one typed
state before ordinary business logic runs.

```rust
use statum::{machine, state, validators, Error};

#[state]
enum TransferState {
    Prepared,
    Authorized(String),
    Settled,
}

#[machine]
struct Transfer<TransferState> {
    id: u64,
}

struct TransferRow {
    id: u64,
    status: &'static str,
    approver: Option<String>,
}

#[validators(Transfer)]
impl TransferRow {
    fn is_prepared(&self) -> statum::Result<()> {
        (self.status == "prepared")
            .then_some(())
            .ok_or(Error::InvalidState)
    }

    fn is_authorized(&self) -> statum::Result<String> {
        if self.status == "authorized" {
            self.approver.clone().ok_or(Error::InvalidState)
        } else {
            Err(Error::InvalidState)
        }
    }

    fn is_settled(&self) -> statum::Result<()> {
        (self.status == "settled")
            .then_some(())
            .ok_or(Error::InvalidState)
    }
}
```

Match the generated machine-scoped enum once at the boundary. After that, pass
concrete typed machines to internal functions.

## Type-State Is Not a Builder

Do not use type-state only to avoid constructor boilerplate. Most builders are
about named arguments, defaults, optional values, and required fields. That is a
construction problem; use `bon`.

Reach for type-state when the object remains alive across meaningful phases and
those phases expose different operations or data.

## Avoid Type-State When

* A plain enum and `match` are clearer.
* The state graph is authored dynamically at runtime.
* All states expose almost the same methods.
* The only goal is requiring constructor fields.
* The generated API would be harder to explain than runtime validation.

## Cautions

* Macro-generated API surface should be documented and tested.
* Runtime boundaries still need validation.
* Persisted state may be corrupt, stale, or version-skewed.
* Overusing type-state can make simple control flow feel abstract.
