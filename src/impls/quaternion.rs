//! Quaternion impls: `Quaternion<T>` is the classic non-commutative
//! division ring over a real field `T` — a ring whose multiplication does
//! not commute, a four-dimensional vector space over `T`, and a normed
//! algebra (the euclidean norm of the four components).
//!
//! The method bodies are hand-written (the Hamiltonian product and the
//! four-component algebra read better as plain `impl`s); the marker levels
//! ride `batch_trait!`.

use batch_impl::batch_trait;

use crate::op::{Additive, Multiplicative};
use crate::quaternion::Quaternion;
use crate::tower::{
    AbelianGroup, ClosedAdd, ClosedMul, DivisionRing, Field, FiniteDimInnerSpace,
    FiniteDimVectorSpace, Group, InnerSpace, Loop, Magma, Module, Monoid, NormedSpace, Quasigroup,
    Real, Ring, Semigroup, Semiring, VectorSpace,
};

// ---- additive side: component-wise ----

impl<T: Magma<Additive>> Magma<Additive> for Quaternion<T> {
    fn combine(&self, rhs: &Self) -> Self {
        Quaternion::new(
            <T as Magma<Additive>>::combine(self.w(), rhs.w()),
            <T as Magma<Additive>>::combine(self.x(), rhs.x()),
            <T as Magma<Additive>>::combine(self.y(), rhs.y()),
            <T as Magma<Additive>>::combine(self.z(), rhs.z()),
        )
    }
}

impl<T: Monoid<Additive>> Monoid<Additive> for Quaternion<T> {
    fn identity() -> Self {
        Quaternion::new(
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
        )
    }
}

impl<T: Group<Additive>> Group<Additive> for Quaternion<T> {
    fn inverse(&self) -> Self {
        Quaternion::new(
            <T as Group<Additive>>::inverse(self.w()),
            <T as Group<Additive>>::inverse(self.x()),
            <T as Group<Additive>>::inverse(self.y()),
            <T as Group<Additive>>::inverse(self.z()),
        )
    }
}

// ---- multiplicative side: Hamilton's product (non-commutative) ----

impl<T: Ring<Additive, Multiplicative>> Magma<Multiplicative> for Quaternion<T> {
    fn combine(&self, rhs: &Self) -> Self {
        Quaternion::new(
            // w1w2 − x1x2 − y1y2 − z1z2
            <T as Magma<Additive>>::combine(
                &<T as Magma<Multiplicative>>::combine(self.w(), rhs.w()),
                &<T as Group<Additive>>::inverse(&<T as Magma<Additive>>::combine(
                    &<T as Magma<Additive>>::combine(
                        &<T as Magma<Multiplicative>>::combine(self.x(), rhs.x()),
                        &<T as Magma<Multiplicative>>::combine(self.y(), rhs.y()),
                    ),
                    &<T as Magma<Multiplicative>>::combine(self.z(), rhs.z()),
                )),
            ),
            // w1x2 + x1w2 + y1z2 − z1y2
            <T as Magma<Additive>>::combine(
                &<T as Magma<Additive>>::combine(
                    &<T as Magma<Multiplicative>>::combine(self.w(), rhs.x()),
                    &<T as Magma<Multiplicative>>::combine(self.x(), rhs.w()),
                ),
                &<T as Magma<Additive>>::combine(
                    &<T as Magma<Multiplicative>>::combine(self.y(), rhs.z()),
                    &<T as Group<Additive>>::inverse(&<T as Magma<Multiplicative>>::combine(
                        self.z(),
                        rhs.y(),
                    )),
                ),
            ),
            // w1y2 − x1z2 + y1w2 + z1x2
            <T as Magma<Additive>>::combine(
                &<T as Magma<Additive>>::combine(
                    &<T as Magma<Multiplicative>>::combine(self.w(), rhs.y()),
                    &<T as Group<Additive>>::inverse(&<T as Magma<Multiplicative>>::combine(
                        self.x(),
                        rhs.z(),
                    )),
                ),
                &<T as Magma<Additive>>::combine(
                    &<T as Magma<Multiplicative>>::combine(self.y(), rhs.w()),
                    &<T as Magma<Multiplicative>>::combine(self.z(), rhs.x()),
                ),
            ),
            // w1z2 + x1y2 − y1x2 + z1w2
            <T as Magma<Additive>>::combine(
                &<T as Magma<Additive>>::combine(
                    &<T as Magma<Multiplicative>>::combine(self.w(), rhs.z()),
                    &<T as Magma<Multiplicative>>::combine(self.x(), rhs.y()),
                ),
                &<T as Magma<Additive>>::combine(
                    &<T as Group<Additive>>::inverse(&<T as Magma<Multiplicative>>::combine(
                        self.y(),
                        rhs.x(),
                    )),
                    &<T as Magma<Multiplicative>>::combine(self.z(), rhs.w()),
                ),
            ),
        )
    }
}

impl<T: Ring<Additive, Multiplicative>> Monoid<Multiplicative> for Quaternion<T> {
    fn identity() -> Self {
        Quaternion::new(
            <T as Monoid<Multiplicative>>::identity(),
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
        )
    }
}

// `Quaternion<T>` is a division ring, never a commutative ring (hence never
// a field): the multiplication is the Hamiltonian product.

impl<T: Field<Additive, Multiplicative> + Copy> DivisionRing<Additive, Multiplicative>
    for Quaternion<T>
{
    fn inv(&self) -> Self {
        let conj = Quaternion::new(
            *self.w(),
            <T as Group<Additive>>::inverse(self.x()),
            <T as Group<Additive>>::inverse(self.y()),
            <T as Group<Additive>>::inverse(self.z()),
        );
        let norm2 = <T as Magma<Additive>>::combine(
            &<T as Magma<Additive>>::combine(
                &<T as Magma<Multiplicative>>::combine(self.w(), self.w()),
                &<T as Magma<Multiplicative>>::combine(self.x(), self.x()),
            ),
            &<T as Magma<Additive>>::combine(
                &<T as Magma<Multiplicative>>::combine(self.y(), self.y()),
                &<T as Magma<Multiplicative>>::combine(self.z(), self.z()),
            ),
        );
        let inv_norm2 = <T as DivisionRing<Additive, Multiplicative>>::inv(&norm2);
        // q⁻¹ = conj(q) / ‖q‖²
        <Quaternion<T> as Magma<Multiplicative>>::combine(
            &conj,
            &Quaternion::new(
                inv_norm2,
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Additive>>::identity(),
            ),
        )
    }
}

// ---- module level: a four-dimensional vector space over `T` ----

impl<T: Field<Additive, Multiplicative>> Module<Additive, Multiplicative> for Quaternion<T> {
    type Scalar = T;

    fn scale(s: &Self::Scalar, v: Self) -> Self {
        Quaternion::new(
            <T as Magma<Multiplicative>>::combine(s, v.w()),
            <T as Magma<Multiplicative>>::combine(s, v.x()),
            <T as Magma<Multiplicative>>::combine(s, v.y()),
            <T as Magma<Multiplicative>>::combine(s, v.z()),
        )
    }
}

// ---- analytic layer: the euclidean norm of the four components ----

impl<T: Real + ClosedAdd + ClosedMul + Copy> NormedSpace<Additive, Multiplicative>
    for Quaternion<T>
{
    type RealField = T;

    fn norm_squared(&self) -> Self::RealField {
        *self.w() * *self.w()
            + *self.x() * *self.x()
            + *self.y() * *self.y()
            + *self.z() * *self.z()
    }

    fn scale_real(&self, r: Self::RealField) -> Self {
        Quaternion::new(*self.w() * r, *self.x() * r, *self.y() * r, *self.z() * r)
    }
}

impl<T: Real + ClosedAdd + ClosedMul + Copy> InnerSpace<Additive, Multiplicative>
    for Quaternion<T>
{
    fn inner_product(&self, other: &Self) -> Self::RealField {
        *self.w() * *other.w()
            + *self.x() * *other.x()
            + *self.y() * *other.y()
            + *self.z() * *other.z()
    }
}

impl<T: Real + ClosedAdd + ClosedMul + Copy> FiniteDimVectorSpace<Additive, Multiplicative>
    for Quaternion<T>
{
    fn dimension() -> usize {
        4
    }

    fn canonical_basis_element(_i: usize) -> Self {
        match _i {
            0 => Quaternion::new(
                <T as Monoid<Multiplicative>>::identity(),
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Additive>>::identity(),
            ),
            1 => Quaternion::new(
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Multiplicative>>::identity(),
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Additive>>::identity(),
            ),
            2 => Quaternion::new(
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Multiplicative>>::identity(),
                <T as Monoid<Additive>>::identity(),
            ),
            3 => Quaternion::new(
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Additive>>::identity(),
                <T as Monoid<Multiplicative>>::identity(),
            ),
            _ => unreachable!(),
        }
    }

    fn dot(&self, other: &Self) -> Self::Scalar {
        self.inner_product(other)
    }
}

// ---- marker levels ----

batch_trait! {
    Semigroup: Semigroup<Additive> <T: @trait<>> Quaternion<T>,
        Semigroup<Multiplicative> <T: Ring<Additive, Multiplicative>> Quaternion<T>;
    Quasigroup: Quasigroup<Additive> <T: @trait<>> Quaternion<T>;
    Loop: Loop<Additive> <T: @trait<>> Quaternion<T>;
    AbelianGroup: AbelianGroup<Additive> <T: @trait<>> Quaternion<T>;
    Semiring: Semiring<Additive, Multiplicative> <T: Ring<Additive, Multiplicative>> Quaternion<T>;
    Ring: Ring<Additive, Multiplicative> <T: @trait<>> Quaternion<T>;
    VectorSpace: VectorSpace<Additive, Multiplicative> <T: Field<Additive, Multiplicative>> Quaternion<T>
        where{Self::Scalar: Field<Additive, Multiplicative>};
    FiniteDimInnerSpace: FiniteDimInnerSpace<Additive, Multiplicative> <T: Real + ClosedAdd + ClosedMul + Copy> Quaternion<T>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::{FiniteDimVectorSpace, NormedSpace};

    #[test]
    fn hamilton_product() {
        let i = Quaternion::new(0.0f64, 1.0, 0.0, 0.0);
        let j = Quaternion::new(0.0, 0.0, 1.0, 0.0);
        let k = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        // i·j = k, j·i = −k (non-commutative!)
        let ij = <Quaternion<f64> as Magma<Multiplicative>>::combine(&i, &j);
        assert_eq!(ij, k);
        let ji = <Quaternion<f64> as Magma<Multiplicative>>::combine(&j, &i);
        assert_eq!(ji, Quaternion::new(0.0, 0.0, 0.0, -1.0));
    }

    #[test]
    fn division_ring_inverse() {
        let q = Quaternion::new(1.0f64, 2.0, 3.0, 4.0);
        let inv = <Quaternion<f64> as DivisionRing<Additive, Multiplicative>>::inv(&q);
        let one = <Quaternion<f64> as Monoid<Multiplicative>>::identity();
        let back = <Quaternion<f64> as Magma<Multiplicative>>::combine(&q, &inv);
        // q·q⁻¹ ≈ 1 (floating point)
        let d = <Quaternion<f64> as Magma<Additive>>::combine(
            &back,
            &<Quaternion<f64> as Group<Additive>>::inverse(&one),
        );
        let _ = d;
        assert!((back.w() - 1.0).abs() < 1e-9);
        assert!(back.x().abs() < 1e-9);
        assert!(back.y().abs() < 1e-9);
        assert!(back.z().abs() < 1e-9);
    }

    #[test]
    fn quaternion_norm() {
        let q = Quaternion::new(1.0f64, 2.0, 2.0, 4.0);
        assert_eq!(q.norm(), 5.0);
        assert_eq!(
            <Quaternion<f64> as FiniteDimVectorSpace<Additive, Multiplicative>>::dimension(),
            4
        );
        // Quaternion<f32> mirrors f64.
        let q32 = Quaternion::new(1.0f32, 2.0, 2.0, 4.0);
        assert_eq!(q32.norm(), 5.0);
        assert_eq!(
            <Quaternion<f32> as FiniteDimVectorSpace<Additive, Multiplicative>>::dimension(),
            4
        );
    }
}
