# User-facing mdBook review checklist

## Request mode and validation boundary

- [ ] The request is classified as Edit or read-only Review under `SKILL.md`.
- [ ] Reviews, audits, checks, and checklist requests report proposed fixes without applying them.
- [ ] Review validation is non-mutating or runs in a safely isolated copy; the original tracked, untracked, and ignored files remain unchanged.
- [ ] Edit-mode builds and tests disable automatic chapter creation, or run in a safely isolated copy; missing chapters outside scope remain reported follow-ups.
- [ ] Build wrappers, preprocessors, include paths, and external side effects were checked before using an isolated copy.
- [ ] Successful checks, failed attempts, static review, and checks not run are reported separately; unchecked items are marked `Not checked`.

## Audience and scope

- [ ] The chapter identifies a user goal or a public behavior.
- [ ] The opening states the outcome or answer.
- [ ] Assumed knowledge and prerequisites are explicit where needed.
- [ ] The chapter does not mix unrelated page types or audiences.
- [ ] Internal details pass the relevance gate and follow the user impact.

## Language and source integrity

- [ ] Original prose uses en-US spelling and punctuation.
- [ ] Commands, identifiers, UI strings, product names, and quotations remain exact.
- [ ] Established public terminology has not been renamed solely for house style.
- [ ] Non-en-US locale trees and translation catalogs are unchanged.
- [ ] Examples use documented public service URLs where appropriate and placeholders for user-specific values; they contain no live credentials, tokens, personal data, or sensitive deployment URLs.
- [ ] Material claims are supported by authoritative sources; conflicts or uncertainty are reported.

## Clarity and concision

- [ ] Headings are specific, sentence case, and properly nested.
- [ ] Paragraphs contain one main idea.
- [ ] Procedures use ordered steps and preserve required sequence.
- [ ] Terms are defined once and used consistently.
- [ ] Vague, promotional, or minimizing language has been removed.
- [ ] Repetition and non-actionable background have been removed.

## Structural density

- [ ] Paragraphs longer than five sentences have been checked for multiple claims, actions, conditions, or decisions.
- [ ] Lists longer than seven to ten items are grouped only when meaningful, converted to an appropriate table, or moved to focused reference material.
- [ ] Procedures longer than ten steps use named phases and useful verification checkpoints when the workflow has natural boundaries.
- [ ] A section with independent prerequisites, workflow, mental model, option catalog, or troubleshooting has been split or linked as a focused page when that improves navigation.
- [ ] One-sentence sections have a real navigation, anchor, or reference purpose; otherwise, they are merged into surrounding prose.
- [ ] Each code example and diagram has one primary lesson or question.
- [ ] Wide or prose-heavy tables have been split, transposed, shortened, or converted back to prose.
- [ ] The common path remains visible and is not buried under advanced variants or edge cases.

## Examples and commands

- [ ] Examples are minimal, safe, and consistent with current behavior.
- [ ] Placeholders are obvious and explained.
- [ ] Code fences have accurate language identifiers.
- [ ] Commands include necessary working-directory or environment context.
- [ ] Verification output is shown only when it helps confirm success.
- [ ] Included snippets use stable anchors when available.

## Markdown and visuals

- [ ] Tables support scanning and have concise cells.
- [ ] Ordered steps are not disguised as tables.
- [ ] Callouts are rare, scoped, and supported by the book.
- [ ] Diagrams are necessary, small, and explained in text.
- [ ] Mermaid or other extended syntax is supported by the book configuration, build tooling, dependencies, and renderer version.
- [ ] Images have meaningful alt text.
- [ ] Raw HTML is avoided or verified.

## mdBook integration

- [ ] `<book-src>/SUMMARY.md` contains the chapter in the intended location, where `<book-src>` comes from `[book].src`.
- [ ] Chapter links use relative `.md` targets when possible.
- [ ] Heading levels do not skip.
- [ ] Filenames and headings preserve stable URLs where practical.
- [ ] The book builds successfully.
- [ ] Link, spelling, and style checks pass when available.
- [ ] Rust examples pass `mdbook test` when applicable.
- [ ] The rendered page has no overflow, broken tables, or unrendered diagram syntax.
- [ ] The final diff contains no unrelated edits, build-created files, or unresolved placeholders.
