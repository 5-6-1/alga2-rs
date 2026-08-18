//! Lie algebras.
//!
//! A Lie algebra is a magma whose bracket is alternating (`[a, a] == 0`) and
//! satisfies the Jacobi identity — the infinitesimal structure behind Lie
//! groups, rotations (the cross product is the 3D example) and the matrix
//! commutator `[A, B] = AB − BA`.

use crate::op::Operator;

use super::Magma;

/// A Lie algebra: a magma with an alternating, Jacobi-satisfying bracket
/// (laws in `crate::laws`).
pub trait LieAlgebra<Op: Operator>: Magma<Op> {
    /// The Lie bracket `[self, other]`.
    fn bracket(&self, other: &Self) -> Self;
}
