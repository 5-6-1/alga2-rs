//! Star semirings (Kleene star).
//!
//! A semiring with a unary `a*` satisfying the closure laws `1 + a·a* == a*`
//! and `1 + a*·a == a*` — the algebraic home of regular languages, path
//! problems in graphs, and the boolean semiring's iteration.

use crate::op::Operator;

use super::Semiring;

/// A semiring with a Kleene star.
pub trait StarSemiring<Oa: Operator, Om: Operator>: Semiring<Oa, Om> {
    /// The Kleene star `a*` (the closure of `a` under the operation).
    fn star(&self) -> Self;
}
