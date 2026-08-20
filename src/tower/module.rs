//! Module and VectorSpace: a ring/field acting on an abelian group.
//!
//! `Module` ties the tower together — the additive group of vectors and the
//! multiplicative monoid of scalars, joined by scalar multiplication
//! (bilinear laws, tested in `crate::laws`). A `VectorSpace` is a module
//! whose scalar ring is a field.
//!
//! The scalar ring is an associated type, so mixed shapes work out:
//! a numeric type is a module over itself (`f64` acting on `f64`), and a
//! tuple of modules over one ring `R` is a module over `R`
//! (`(R, R)`-style componentwise scaling).

use crate::op::Operator;

use super::{AbelianGroup, Field, Ring};

/// A module over a ring: an additive abelian group with a compatible scalar
/// multiplication.
///
/// The laws (tested in [`crate::laws`]) are bilinearity — `s·(u+v) =
/// s·u + s·v`, `(s+t)·v = s·v + t·v`, `(s·t)·v = s·(t·v)`, `1·v = v` —
/// where `+`/`·` are the operator-parameterized operations.
pub trait Module<Oa: Operator, Om: Operator>: AbelianGroup<Oa> {
    /// The scalar ring acting on `Self`.
    type Scalar: Ring<Oa, Om>;

    /// Scalar multiplication `s · v`.
    fn scale(s: &Self::Scalar, v: Self) -> Self;
}

/// A vector space: a module whose scalar ring is a field.
pub trait VectorSpace<Oa: Operator, Om: Operator>: Module<Oa, Om>
where
    Self::Scalar: Field<Oa, Om>,
{
}

/// A free module: a module with a finite basis.
pub trait FreeModule<Oa: Operator, Om: Operator>: Module<Oa, Om> {
    /// The rank — the number of basis elements.
    fn rank() -> usize;

    /// The `i`-th basis element.
    fn basis_element(_i: usize) -> Self;

    /// The coordinate of `self` along the `i`-th basis element.
    fn coordinate(&self, i: usize) -> Self::Scalar;
}

/// A linear map between two vector spaces over the same field.
///
/// The linearity laws (`linear_map_additive`, `linear_map_scalar`) are in
/// `crate::laws`. Concrete matrices implement this; the tower only declares
/// the shape.
pub trait LinearMap<Oa: Operator, Om: Operator>
where
    <Self::Domain as Module<Oa, Om>>::Scalar: Field<Oa, Om>,
{
    /// The domain vector space.
    type Domain: VectorSpace<Oa, Om>;

    /// The codomain vector space (over the same scalar field).
    type Codomain: VectorSpace<Oa, Om, Scalar = <Self::Domain as Module<Oa, Om>>::Scalar>;

    /// Applies the map to `v`.
    fn apply(&self, v: &Self::Domain) -> Self::Codomain;
}
