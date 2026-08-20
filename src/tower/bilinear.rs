//! Bilinear forms: `B: V × V → F`, linear in each argument.
//!
//! A form is a separate object from the space it acts on (a Gram matrix, an
//! inner product, a metric tensor). The hierarchy declares the shape;
//! `crate::laws` tests the bilinearity, symmetry, and positive-definiteness
//! axioms. The operator pair is fixed to the additive/multiplicative one —
//! the only one with a meaningful notion of scalar multiplication.

use crate::op::{Additive, Multiplicative};
use crate::tower::{Field, Module, VectorSpace};

/// A bilinear form over a vector space: `B(u, v)` linear in both arguments
/// (the bilinearity laws are in the `crate::laws` module, feature `proptest`).
pub trait BilinearForm
where
    <Self::Space as Module<Additive, Multiplicative>>::Scalar: Field<Additive, Multiplicative>,
{
    /// The vector space the form acts on.
    type Space: VectorSpace<Additive, Multiplicative>;

    /// The scalar field the form evaluates into.
    type Scalar: Field<Additive, Multiplicative>;

    /// Evaluates `B(u, v)`.
    fn apply(&self, u: &Self::Space, v: &Self::Space) -> Self::Scalar;
}

/// A symmetric bilinear form: `B(u, v) = B(v, u)` (a law).
pub trait SymmetricBilinearForm: BilinearForm {}

/// A positive-definite bilinear form: `B(v, v) > 0` for every `v ≠ 0`
/// (a law, which requires an ordered scalar field). An inner product is the
/// canonical example.
pub trait PositiveDefinite: SymmetricBilinearForm {}
