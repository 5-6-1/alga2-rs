//! Normed spaces, inner-product spaces, and finite-dimensional vector
//! spaces — the analytic layer over [`VectorSpace`].
//!
//! The norm lives in a real field ([`Real`]); inner
//! products are **real** (euclidean), so a complex vector space's norm is
//! still real. Gram–Schmidt orthogonalization is implemented once here and
//! inherited by every finite-dimensional inner-product space.

use crate::op::Operator;

use super::real::Real;
use super::{DivisionRing, Field, Group, Magma, VectorSpace};

/// A normed vector space: a [`VectorSpace`] with a norm over a real field.
pub trait NormedSpace<Oa: Operator, Om: Operator>: VectorSpace<Oa, Om>
where
    Self::Scalar: Field<Oa, Om>,
{
    /// The real field the norm lives in.
    type RealField: Real;

    /// `‖v‖²`.
    fn norm_squared(&self) -> Self::RealField;

    /// `‖v‖`.
    fn norm(&self) -> Self::RealField {
        self.norm_squared().sqrt()
    }

    /// Real-scalar multiplication `v·r` — the norm's field acting on the
    /// vector. The identity on the numerics (`Scalar = RealField`), the
    /// natural action on complex vectors.
    fn scale_real(&self, r: Self::RealField) -> Self
    where
        Self: Sized;

    /// `v / ‖v‖` — `NaN` on the zero vector.
    fn normalize(&self) -> Self
    where
        Self: Sized,
    {
        self.scale_real(self.norm().inv())
    }

    /// `v / ‖v‖`, `None` when the norm is not strictly greater than `eps`.
    fn try_normalize(&self, eps: Self::RealField) -> Option<Self>
    where
        Self: Sized,
    {
        let n = self.norm();
        if n > eps { Some(self.scale_real(n.inv())) } else { None }
    }
}

/// An inner-product space: a [`NormedSpace`] with a positive-definite
/// bilinear form (implementations must satisfy `norm_squared == ⟨v, v⟩`).
pub trait InnerSpace<Oa: Operator, Om: Operator>: NormedSpace<Oa, Om>
where
    Self::Scalar: Field<Oa, Om>,
{
    /// The (real, euclidean) inner product `⟨self, other⟩`.
    fn inner_product(&self, other: &Self) -> Self::RealField;

    /// The angle between `self` and `other` (radians).
    fn angle(&self, other: &Self) -> Self::RealField
    where
        Self: Sized,
    {
        // RealField is `ClosedMul`/`ClosedDiv`, so the standard operators
        // apply (the norm's field is real).
        let cos = self.inner_product(other) / (self.norm() * other.norm());
        cos.acos()
    }
}

/// A finite-dimensional vector space: a dimension and a canonical basis.
pub trait FiniteDimVectorSpace<Oa: Operator, Om: Operator>: VectorSpace<Oa, Om>
where
    Self::Scalar: Field<Oa, Om>,
{
    /// The dimension of the space.
    fn dimension() -> usize;

    /// The `i`-th canonical basis vector.
    fn canonical_basis_element(i: usize) -> Self;

    /// Visits every canonical basis vector until `f` returns `true`.
    fn canonical_basis<F: FnMut(&Self) -> bool>(mut f: F)
    where
        Self: Sized,
    {
        for i in 0..Self::dimension() {
            if f(&Self::canonical_basis_element(i)) {
                break;
            }
        }
    }

    /// The coordinate-wise dot product `⟨self, other⟩` in the field.
    fn dot(&self, other: &Self) -> Self::Scalar;
}

/// A finite-dimensional inner-product space: Gram–Schmidt orthogonalization.
pub trait FiniteDimInnerSpace<Oa: Operator, Om: Operator>:
    FiniteDimVectorSpace<Oa, Om> + InnerSpace<Oa, Om>
where
    Self::Scalar: Field<Oa, Om>,
{
    /// Modified Gram–Schmidt: orthonormalizes the vectors in place (the
    /// linearly independent prefix), returning the count of output vectors.
    fn orthonormalize(vs: &mut [Self]) -> usize
    where
        Self: Copy,
    {
        // `Real` has a `Default` (the zero of the real field).
        let zero = Self::RealField::default();
        let mut k = 0;
        for i in 0..vs.len() {
            let mut v = vs[i];
            for u in vs[..k].iter() {
                // v -= ⟨v, uⱼ⟩·uⱼ
                let proj = v.inner_product(u);
                let term = u.scale_real(proj);
                v = <Self as Magma<Oa>>::combine(&v, &<Self as Group<Oa>>::inverse(&term));
            }
            let n = v.norm();
            if n > zero {
                vs[k] = v.scale_real(n.inv());
                k += 1;
            }
        }
        k
    }
}
