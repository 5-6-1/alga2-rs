# Contributing to alga2

Thanks for your interest! This document describes how to work on the crate.
It is short on purpose — the two sections below are the ones that matter.

## Ground rules

- **Zero duplication is a hard requirement.** Every method body is written
  exactly once. If a change would write a body a second time, it must go
  through the batch-impl DSL instead (`batch_trait!` first, `batch_impl_only`
  where the directive system earns its keep) or into a shared helper.
- **batch-impl's source is off-limits.** This repository consumes
  batch-impl; the DSL's owner maintains it. If the DSL cannot express what
  the matrix needs, either work around it in alga2 or file an issue — do not
  patch the dependency.
- **New types enter the matrix, not the code.** Adding a type to a family
  is one line in the relevant `batch_trait!` block (`@int`, `@num`, `@pri`,
  …), not a new impl. Keep it that way.
- **The tower is the single source of truth.** `src/tower` defines every
  trait; impls only implement. New levels go in `src/tower/<name>.rs` and
  are re-exported from `src/tower/mod.rs`.
- **No unsafe, no new runtime dependencies.** The core is `no_std`;
  `alloc`/`std` containers layer on via features. A new dependency needs a
  justification in the PR description.

## The quality gate

Every change must pass, before opening a PR:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo test --features proptest
cargo check --no-default-features
cargo check --no-default-features --features alloc
```

- MSRV is 1.93 (edition 2024). CI runs the suite on stable and on MSRV.
- `#![deny(missing_docs)]` is on: new public items need `///` docs.
- Law testing: a new hierarchy level ships its laws in `src/laws` and the
  matrix is property-tested through them (see `src/laws/mod.rs`).

## Working with the DSL

The impl matrices live in `src/impls` and use batch-impl's `batch_trait!`
(and, sparingly, `batch_impl_only`). The module doc of `src/impls/mod.rs`
explains the three entry points and the reuse rules. `docs/DESIGN.md` covers
the syntax idioms in depth — read it before touching the matrices.

Practical checks while editing DSL:

- After a `batch_trait!` edit, `cargo check` surfaces most syntax problems;
  the expanded impls are the source of truth for what was actually emitted.
- `<T as Trait<>>` (explicit empty brackets) and `<T as Trait>` are **not**
  interchangeable inside a spec body: the empty-bracket form follows the
  spec's operator arguments, the bare form uses the defaults. See the
  `@with_mul` combine in `src/impls/complex.rs` for a live example.
- When in doubt, expand: `cargo expand --lib impls::<file>` (nightly) or
  read the generated code via `cargo doc --document-private-items`.

## Commit messages

Conventional Commits, imperative, ≤ 50 chars: `feat:` / `fix:` /
`refactor:` / `perf:` / `test:` / `docs:` / `chore:` / `build:`. No scope
for single-crate changes.

## Documentation

- User-facing docs are English (README, docs/). Comments and doc comments
  are English too.
- Design decisions worth keeping land in `docs/DESIGN.md` or the roadmap
  (`docs/ROADMAP.md`); API migration notes in `docs/ALGA-DIFF.md`.
- Changelog: every user-visible change gets a `CHANGELOG.md` entry under
  `[Unreleased]`.

## Testing

- Default: inline `#[cfg(test)] mod tests` per source file.
- Law matrices are proptest-driven in `src/laws/mod.rs`; move to the
  standalone `tests/` directory only when a test is integration-shaped
  (`tests/derive_alga.rs` is the example).
