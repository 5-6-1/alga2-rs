# Security Policy

## Supported versions

Only the latest release of `alga2` receives security fixes. There are no
long-term-support branches.

## Reporting a vulnerability

Please **do not** open a public issue for a security vulnerability. Report
it privately instead:

- Email the maintainer(s) — see the `authors` field in `Cargo.toml`.
- If the fix is straightforward, a private branch/patch sent alongside the
  report is welcome.

You will receive an acknowledgement within a few days and a target timeline
for the fix.

## Scope and posture

- The crate is `#![forbid(unsafe_code)]`: there is no `unsafe` anywhere, by
  construction. Memory-safety reports are therefore out of scope by design;
  correctness and soundness-of-generics issues (coherence, law violations,
  panic-on-valid-input) are in scope.
- The core is `no_std` with zero runtime dependencies; the only dependency
  surface is the build-time `batch-impl` DSL and the optional `proptest`
  (dev/testing) — neither is linked into downstream builds.
- Integer impls use wrapping arithmetic (mod 2^N) deliberately: an
  "overflow" in debug mode that crashes a downstream program where the
  algebra expects a residue is reportable; a panic in `ModN::new` on an
  out-of-range value is by contract, not a bug.

## Disclosures

Security fixes are noted in `CHANGELOG.md` with the severity and affected
versions.
