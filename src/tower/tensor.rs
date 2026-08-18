//! Tensor products: the universal target of bilinear maps.
//!
//! A tensor product `V ⊗ W` receives the universal bilinear map
//! `V × W → V ⊗ W`. Concrete tensor types (multidimensional arrays, outer
//! products) belong to linear-algebra libraries; the tower declares the
//! shape, and `crate::laws` tests the bilinearity axioms.

use crate::op::Operator;
use crate::tower::Magma;

/// A tensor product `V ⊗ W`: the target of the universal bilinear map
/// `V × W → V ⊗ W`.
///
/// The bilinearity laws (`tensor_bilinear_left`/`tensor_bilinear_right`) are
/// in [`crate::laws`].
pub trait TensorProduct<Op: Operator>: Sized {
    /// The left factor.
    type Left: Magma<Op>;

    /// The right factor.
    type Right: Magma<Op>;

    /// The tensor product of `left` and `right`.
    fn tensor_product(left: Self::Left, right: Self::Right) -> Self;
}
