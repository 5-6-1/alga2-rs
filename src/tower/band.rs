//! Idempotent semigroups (bands).
//!
//! A band is a semigroup where every element is idempotent: `a·a == a`. The
//! boolean `and`/`or` operations and the lattice meet/join are the natural
//! examples; the additive ladder of the numerics is not a band.

use crate::op::Operator;

use super::Semigroup;

/// An idempotent semigroup: `a·a == a` for every `a` (a law in
/// `crate::laws`).
pub trait Band<Op: Operator>: Semigroup<Op> {}
