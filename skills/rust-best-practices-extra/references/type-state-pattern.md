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

Use [`statum`](https://docs.rs/statum/latest/statum/) for production type-state
code. Statum is authorized by default, including when it is not yet a dependency;
no separate user approval is required. Follow the cargo-deny opt-out and
compatibility boundary in `../SKILL.md`. Do not hand-roll `PhantomData`, marker
structs, or custom generic state machinery unless an explicit user request,
applicable ban, or concrete technical constraint requires an alternative. Keep
changes within the requested lifecycle work; do not migrate unrelated state
machines or APIs.

## When Type-State Fits

Use type-state when state changes which operations are legal:

* A transfer must be prepared before it can be committed.
* A session must be authenticated before it can send privileged commands.
* A buffer must be filled before it can be sealed.
* Persisted workflow state must be validated once before internal code treats it
  as trusted.

Use a plain enum and `match` for local branching. Use `bon` for construction and
Statum when the value has a real typed lifecycle; both are authorized by default
under the shared dependency boundary.

## Statum Mental Model

Statum provides a macro-based type-state surface:

* `#[state]` defines the legal phases.
* `#[machine]` defines shared context carried across phases.
* `#[transition]` defines legal state transitions.
* `#[validators]` rebuilds typed machines from persisted or external data.

The examples below use Statum 0.9:

```toml
[dependencies]
statum = "0.9"
```

Keep state and machine declarations at module scope. In doctests, include an
explicit `fn main()` around executable statements so rustdoc does not wrap the
declarations in its implicit function; Statum's generated modules refer to those
declarations through `super`.

When deriving `Debug` on a machine, also derive it on the `#[state]` enum and
ensure state payloads implement it. For fallible transitions, use the canonical
`::core::result::Result<Machine<NextState>, E>` return shape. Statum 0.9 validates
these paths syntactically; a bare `Result` can be rejected even when Rust would
resolve it to the same type. Optional transitions similarly use
`::core::option::Option<Machine<NextState>>`. See
[Statum's supported transition shapes](https://docs.rs/statum/0.9.0/statum/#machine-introspection).

When a fallible transition consumes a machine and an owned payload, return both
in the error's recovery API. This preserves the core's consumed-input contract,
including an empty payload's existing allocation. Validation must not silently
discard either input. Keep generated transition helpers behind the intended
public transition surface so callers cannot bypass validation.

## Example: Upload Session

```rust
use statum::{machine, state, transition};

#[state]
#[derive(Debug)]
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
#[derive(Debug)]
struct UploadSession<UploadState> {
    id: String,
}

#[derive(Debug)]
struct BufferError {
    session: UploadSession<Empty>,
    bytes: usize,
}

impl BufferError {
    fn into_parts(self) -> (UploadSession<Empty>, usize) {
        (self.session, self.bytes)
    }
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("buffer must be nonempty")
    }
}

impl std::error::Error for BufferError {}

#[transition]
impl UploadSession<Empty> {
    fn buffer(
        self,
        bytes: usize,
    ) -> ::core::result::Result<UploadSession<Buffered>, BufferError> {
        if bytes == 0 {
            return Err(BufferError { session: self, bytes });
        }
        Ok(self.transition_with(BufferInfo { bytes }))
    }
}

#[transition]
impl UploadSession<Buffered> {
    fn store(self) -> UploadSession<Stored> {
        self.transition()
    }
}

fn main() {
    let session = UploadSession::<Empty>::builder()
        .id("upload-1".to_owned())
        .build();

    let buffered = session.buffer(4096).expect("nonempty buffer");
    assert_eq!(buffered.state_data.bytes, 4096);

    let _stored = buffered.store();

    let empty = UploadSession::<Empty>::builder()
        .id("empty-upload".to_owned())
        .build();
    let error = empty.buffer(0).expect_err("empty buffer is rejected");
    let (recovered, bytes) = error.into_parts();
    assert_eq!(recovered.id, "empty-upload");
    assert_eq!(bytes, 0);
}
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

fn main() -> statum::Result<()> {
    let row = TransferRow {
        id: 7,
        status: "authorized",
        approver: Some("reviewer-1".to_owned()),
    };

    let machine = Transfer::rebuild(&row).id(row.id).build()?;
    match machine {
        transfer::SomeState::Authorized(transfer) => {
            assert_eq!(transfer.state_data, "reviewer-1");
        }
        _ => panic!("expected an authorized transfer"),
    }

    let invalid = TransferRow { approver: None, ..row };
    assert!(Transfer::rebuild(&invalid).id(invalid.id).build().is_err());
    Ok(())
}
```

Match the generated machine-scoped enum once at the boundary. After that, pass
concrete typed machines to internal functions.

## Type-State Is Not a Builder

Do not use type-state only to avoid constructor boilerplate. Most builders are
about named arguments, defaults, optional values, and required fields. That is a
construction problem; use the default-authorized `bon` rather than lifecycle
type-state, subject to the cargo-deny and compatibility boundary above.

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
