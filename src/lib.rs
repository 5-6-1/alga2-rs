//! The full crate documentation — hierarchy overview, quick start, law
//! testing — is the README, rendered into this page:

#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
// batch-impl's repeat reductions (`@(1+)..0`) legitimately emit `x + 0`
// folds: the zero is the algebra's identity seed, not a no-op. clippy's
// identity_op cannot see past the macro expansion, so it is allowed
// crate-wide; the folds are intentional.
#![allow(clippy::identity_op)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod complex;
// Exported law-testing strategies: always available to this crate's own
// tests, gated behind the `proptest` feature for downstream users (proptest
// is std-only and must never leak into the bare core build).
#[cfg(any(test, feature = "proptest"))]
pub mod laws;
pub mod modn;
pub mod op;
pub mod quaternion;
pub mod tower;

mod impls;

// `impls` is intentionally private: it only contains trait implementations
// over the public types; users interact with the hierarchy through the
// prelude-style re-exports in `tower` and `op`.
