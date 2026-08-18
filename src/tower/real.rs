//! Real numbers: ordered fields with `sqrt` and `abs`.
//!
//! The scalar type of the normed spaces ([`NormedSpace`](super::norm::NormedSpace))
//! and the common ground of the geometric traits. Real fields are also
//! closed under the std `*`/`/` operators (the norm/angle arithmetic uses
//! them directly).

use crate::op::{Additive, Multiplicative};

use super::Field;
use super::closed::{ClosedDiv, ClosedMul};

/// An ordered field with `sqrt` and `abs`, closed under `*` and `/`.
pub trait Real:
    Field<Additive, Multiplicative>
    + ClosedMul<Self>
    + ClosedDiv<Self>
    + Default
    + PartialOrd
    + core::ops::Neg<Output = Self>
{
    /// The square root (rounding for non-perfect squares).
    fn sqrt(self) -> Self;

    /// The absolute value.
    fn abs(self) -> Self;

    /// The arccosine (used by the inner-product angle).
    fn acos(self) -> Self;
}

/// A field of complex numbers over a real field.
pub trait ComplexField: Field<Additive, Multiplicative> {
    /// The real field the complex numbers are built over.
    type RealField: Real;

    /// The embedding of a real number (the real axis).
    fn from_real(re: Self::RealField) -> Self;

    /// The real part.
    fn re(&self) -> Self::RealField;

    /// The imaginary part.
    fn im(&self) -> Self::RealField;

    /// The complex conjugate.
    fn conjugate(&self) -> Self;
}
