//! Polynomials over a ring: a finite coefficient sequence.
//!
//! The tower declares the shape and coefficient access; concrete polynomial
//! types (dense, sparse, fixed-length `[C; N]` arrays) implement it. A
//! polynomial ring inherits the tower from its coefficient sequence — the
//! array/`Vec` impls in `crate::impls`.

use crate::op::{Additive, Multiplicative};
use crate::tower::Ring;

/// A polynomial `Σ aᵢ·xⁱ` over a coefficient ring: a finite coefficient
/// sequence with the usual degree notion.
pub trait Polynomial: Sized + Clone {
    /// The coefficient ring.
    type Coefficient: Ring<Additive, Multiplicative>;

    /// The degree — `max { i | aᵢ ≠ 0 }`, or `0` for the zero polynomial.
    fn degree(&self) -> usize;

    /// The `i`-th coefficient `aᵢ` (`0` beyond the degree).
    fn coefficient(&self, i: usize) -> Self::Coefficient;
}
