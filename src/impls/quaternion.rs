//! Quaternion impls: `Quaternion<T>` is the classic non-commutative
//! division ring over a real field `T` — a ring whose multiplication does
//! not commute, a four-dimensional vector space over `T`, and a normed
//! algebra (the euclidean norm of the four components).
//!
//! The simple component-wise levels ride `batch_trait!`; the bulky algebra
//! (Hamiltonian product, division-ring inverse, the normed layer) is
//! hand-written.

use batch_impl::batch_trait;

use crate::op::{Additive, Multiplicative};
use crate::quaternion::Quaternion;
use crate::tower::{
    AbelianGroup, ClosedAdd, ClosedMul, DivisionRing, Field, FiniteDimInnerSpace,
    FiniteDimVectorSpace, Group, InnerSpace, Loop, Magma, Module, Monoid, NormedSpace, Quasigroup,
    Real, Ring, Semigroup, Semiring, VectorSpace,
};

// ---- simple levels: component-wise, one batch_trait! block ----

batch_trait! {
    @am=Additive, Multiplicative;
    @tr_add=@trait<Additive> <T: @trait<>> Quaternion<T>;
    @tr_mul=@trait<Multiplicative> <T: Ring<@am>> Quaternion<T>;
    @tr_am=@trait<@am> <T: Ring<>> Quaternion<T>;
    Magma: @tr_add{
        fn combine(&self, rhs: &Self) -> Self {
            Quaternion::new(
                <T as Magma>::combine(self.w(), rhs.w()),
                <T as Magma>::combine(self.x(), rhs.x()),
                <T as Magma>::combine(self.y(), rhs.y()),
                <T as Magma>::combine(self.z(), rhs.z()),
            )
        }
    };
    Semigroup: @tr_add,@tr_mul;
    Monoid: @tr_add{
        fn identity() -> Self {
            Quaternion::new(
                <T as Monoid>::identity(),
                <T as Monoid>::identity(),
                <T as Monoid>::identity(),
                <T as Monoid>::identity(),
            )
        }
    },
        @tr_mul{
        fn identity() -> Self {
            Quaternion::new(
                <T as Monoid<Multiplicative>>::identity(),
                <T as Monoid>::identity(),
                <T as Monoid>::identity(),
                <T as Monoid>::identity(),
            )
        }
    };
    Quasigroup: @tr_add;
    Loop: @tr_add;
    Group: @tr_add{
        fn inverse(&self) -> Self {
            Quaternion::new(
                <T as Group>::inverse(self.w()),
                <T as Group>::inverse(self.x()),
                <T as Group>::inverse(self.y()),
                <T as Group>::inverse(self.z()),
            )
        }
    };
    AbelianGroup: @tr_add;
    Semiring: @tr_am;
    Ring: @trait<@am> <T: @trait<>> Quaternion<T>;
    Module: @trait<@am> <T: Field<>> Quaternion<T>{
        type Scalar = T;
        fn scale(s: &Self::Scalar, v: Self) -> Self {
            Quaternion::new(
                <T as Magma<Multiplicative>>::combine(s, v.w()),
                <T as Magma<Multiplicative>>::combine(s, v.x()),
                <T as Magma<Multiplicative>>::combine(s, v.y()),
                <T as Magma<Multiplicative>>::combine(s, v.z()),
            )
        }
    };
    VectorSpace: @trait<@am> <T: Field<>> Quaternion<T> where Self::Scalar: Field<>;
    FiniteDimInnerSpace: @trait<@am> <T: Real + ClosedAdd + ClosedMul + Copy> Quaternion<T>;
    // Division ring inverse, the normed layer: structurally similar to the
    // complex versions, well under the 60-line reuse boundary.
    DivisionRing: @trait<@am> <T: Field<> + Copy> Quaternion<T>{
        fn inv(&self) -> Self {
            let conj = Quaternion::new(
                *self.w(),
                <T as Group>::inverse(self.x()),
                <T as Group>::inverse(self.y()),
                <T as Group>::inverse(self.z()),
            );
            let norm2 = <T as Magma>::combine(
                &<T as Magma>::combine(
                    &<T as Magma<Multiplicative>>::combine(self.w(), self.w()),
                    &<T as Magma<Multiplicative>>::combine(self.x(), self.x()),
                ),
                &<T as Magma>::combine(
                    &<T as Magma<Multiplicative>>::combine(self.y(), self.y()),
                    &<T as Magma<Multiplicative>>::combine(self.z(), self.z()),
                ),
            );
            let inv_norm2 = <T as DivisionRing<>>::inv(&norm2);
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
    };
    NormedSpace: @trait<@am,RealField = T> <T: Real + ClosedAdd + ClosedMul + Copy> Quaternion<T>{
        fn norm_squared(&self) -> Self::RealField {
            *self.w() * *self.w()
                + *self.x() * *self.x()
                + *self.y() * *self.y()
                + *self.z() * *self.z()
        }
        fn scale_real(&self, r: Self::RealField) -> Self {
            Quaternion::new(*self.w() * r, *self.x() * r, *self.y() * r, *self.z() * r)
        }
    };
    InnerSpace: @trait<@am> <T: Real + ClosedAdd + ClosedMul + Copy> Quaternion<T>{
        fn inner_product(&self, other: &Self) -> Self::RealField {
            *self.w() * *other.w()
                + *self.x() * *other.x()
                + *self.y() * *other.y()
                + *self.z() * *other.z()
        }
    };
    FiniteDimVectorSpace: @trait<@am> <T: Real + ClosedAdd + ClosedMul + Copy> Quaternion<T>{
        fn dimension() -> usize { 4 }
        fn canonical_basis_element(_i: usize) -> Self {
            match _i {
                0 => Quaternion::new(
                    <T as Monoid<Multiplicative>>::identity(),
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Additive>>::identity()),
                1 => Quaternion::new(
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Multiplicative>>::identity(),
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Additive>>::identity()),
                2 => Quaternion::new(
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Multiplicative>>::identity(),
                    <T as Monoid<Additive>>::identity()),
                3 => Quaternion::new(
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Additive>>::identity(),
                    <T as Monoid<Multiplicative>>::identity()),
                _ => unreachable!(),
            }
        }
        fn dot(&self, other: &Self) -> Self::Scalar {
            self.inner_product(other)
        }
    };
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

// `Quaternion<T>` is a division ring, never a commutative ring (hence never
// a field): the multiplication is the Hamiltonian product. Its 55-line body
// is the one genuinely bulky algorithm — hand-written.

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
