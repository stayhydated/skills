# Internal mdBook review checklist

## Request mode and validation boundary

- [ ] The request is classified as Edit or read-only Review under `SKILL.md`.
- [ ] Reviews, audits, checks, and checklist requests report proposed fixes without applying them.
- [ ] Review validation is non-mutating or runs in a safely isolated copy; the original tracked, untracked, and ignored files remain unchanged.
- [ ] Edit-mode builds and tests disable automatic chapter creation, or run in a safely isolated copy; missing chapters outside scope remain reported follow-ups.
- [ ] Build wrappers, preprocessors, include paths, and external side effects were checked before using an isolated copy.
- [ ] Successful checks, failed attempts, static review, and checks not run are reported separately; unchecked items are marked `Not checked`.

## Scope and truth

- [ ] The chapter names its audience and the maintenance decision it supports.
- [ ] Contract, invariant, current implementation, rationale, limitation, and proposal are not conflated.
- [ ] Material claims are verified against code, tests, schemas, configuration, or runtime evidence.
- [ ] Uncertainty and version dependence are explicit.
- [ ] Incidental implementation detail has been removed.

## Language and source integrity

- [ ] Original prose uses en-US spelling and punctuation.
- [ ] Source symbols, configuration keys, log text, protocol terms, product names, and quotations remain exact.
- [ ] Established domain terminology has not been renamed solely for house style.
- [ ] Non-en-US locale trees and translation catalogs are unchanged.
- [ ] Material claims have traceable evidence; conflicts, inference, and uncertainty are explicit.
- [ ] Examples and source excerpts contain no secrets, credentials, personal data, or unsafe production values.

## Architecture and behavior

- [ ] Responsibilities and non-responsibilities are clear.
- [ ] Inputs, outputs, state ownership, and collaborators are identified.
- [ ] Principal control or data flow reaches a meaningful completion point.
- [ ] Concurrency, ordering, idempotency, and transaction boundaries are covered when relevant.
- [ ] Preconditions, postconditions, and invariants are explicit.
- [ ] Failure propagation, retry, cleanup, and partial success are documented.
- [ ] Security, compatibility, performance, and observability implications are covered when relevant.
- [ ] Extension points name constraints and required tests.

## Prose and structure

- [ ] The opening states scope and the main conclusion.
- [ ] Each paragraph makes one primary engineering claim.
- [ ] Terms and component names match the implementation.
- [ ] Constraints appear near the behavior they constrain.
- [ ] The chapter is organized by responsibility and behavior, not repository traversal.
- [ ] Rationale explains forces and consequences rather than retelling chronology.

## Structural density

- [ ] Paragraphs longer than five sentences have been checked for conflated claims or kinds of truth.
- [ ] The page does not mix abstraction levels or unrelated maintenance decisions without an explicit reason.
- [ ] Lists longer than seven to ten items use a verified grouping such as ownership, lifecycle, failure class, or risk, or are moved to focused reference material.
- [ ] Procedures longer than ten steps use phases, verification points, and rollback boundaries when the workflow supplies them.
- [ ] A subsystem, lifecycle, or flow with its own contract, state model, operational signals, failure behavior, or change guide has a focused page when appropriate.
- [ ] One-sentence sections have a stable anchor, decision-status, or navigation purpose; otherwise, they are merged.
- [ ] Each table has one comparison dimension, each code sample establishes one primary contract or extension pattern, and each diagram answers one engineering question.
- [ ] The principal flow remains visible; edge cases and exhaustive detail do not dominate it.

## Tables, code, and diagrams

- [ ] Tables have a clear comparison dimension and concise cells.
- [ ] Code snippets establish a contract, invariant, schema, or extension pattern.
- [ ] Source includes use stable anchors when available.
- [ ] Diagrams answer a specific question and stay at one abstraction level.
- [ ] Diagram edges, asynchronous boundaries, persistence, and external systems are labeled where material.
- [ ] Mermaid or other extended syntax is supported by the book configuration, build tooling, dependencies, and renderer version.
- [ ] Every diagram has a textual conclusion or associated invariants.

## mdBook and validation

- [ ] `<book-src>/SUMMARY.md` places the chapter in the expected subsystem or concern, where `<book-src>` comes from `[book].src`.
- [ ] Relative `.md` links resolve.
- [ ] Heading hierarchy is valid and filenames remain stable where practical.
- [ ] The book builds successfully.
- [ ] Repository link, spelling, style, schema, and Markdown checks pass when available.
- [ ] Testable Rust examples pass `mdbook test` when applicable.
- [ ] Non-Rust examples and schemas are checked with project-native tooling.
- [ ] Rendered tables and diagrams fit the content width and remain legible.
- [ ] The final report names evidence consulted and validation performed.
- [ ] The final diff contains no unrelated edits, build-created files, or unresolved placeholders.
