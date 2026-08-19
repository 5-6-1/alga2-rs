//! Quaternion impls: `Quaternion<T>` is the classic non-commutative
//! division ring over a real field `T` — a ring whose multiplication does
//! not commute, a four-dimensional vector space over `T`, and a normed
//! algebra (the euclidean norm of the four components).

use batch_impl::{batch_impl_only, batch_trait};

use crate::op::{Additive, Multiplicative};
use crate::quaternion::Quaternion;
use crate::tower::{
    AbelianGroup, ClosedAdd, ClosedMul, DivisionRing, Field, FiniteDimInnerSpace,
    FiniteDimVectorSpace, Group, InnerSpace, Loop, Magma, Module, Monoid, NormedSpace, Quasigroup,
    Real, Ring, Semigroup, Semiring, VectorSpace,
};

// ---- additive side: component-wise ----

#[batch_impl_only(
    Magma<Additive> <T: @trait<>> Quaternion<T> #combine{
        Quaternion::new(
            <T as Magma<Additive>>::combine(self.w(), rhs.w()),
            <T as Magma<Additive>>::combine(self.x(), rhs.x()),
            <T as Magma<Additive>>::combine(self.y(), rhs.y()),
            <T as Magma<Additive>>::combine(self.z(), rhs.z()),
        )
    },
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    Monoid<Additive> <T: @trait<>> Quaternion<T> #identity{
        Quaternion::new(
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
        )
    },
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

#[batch_impl_only(
    Group<Additive> <T: @trait<>> Quaternion<T> #inverse{
        Quaternion::new(
            <T as Group<Additive>>::inverse(self.w()),
            <T as Group<Additive>>::inverse(self.x()),
            <T as Group<Additive>>::inverse(self.y()),
            <T as Group<Additive>>::inverse(self.z()),
        )
    },
)]
trait Group<Op: Operator>: Loop<Op> {
    fn inverse(&self) -> Self;
}

// ---- multiplicative side: Hamilton's product (non-commutative) ----

#[batch_impl_only(
    Magma<Multiplicative> <T: Ring<Additive, Multiplicative>> Quaternion<T> #combine{
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
    },
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    Monoid<Multiplicative> <T: Ring<Additive, Multiplicative>> Quaternion<T> #identity{
        Quaternion::new(
            <T as Monoid<Multiplicative>>::identity(),
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
            <T as Monoid<Additive>>::identity(),
        )
    },
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

// `Quaternion<T>` is a division ring, never a commutative ring (hence never
// a field): the multiplication is the Hamiltonian product.

#[batch_impl_only(
    DivisionRing<Additive, Multiplicative> <T: Field<Additive, Multiplicative> + Copy> Quaternion<T> #inv{
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
            &Quaternion::new(inv_norm2, <T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity()),
        )
    },
)]
trait DivisionRing<Oa: Operator, Om: Operator>: Ring<Oa, Om> {
    fn inv(&self) -> Self;
}

// ---- module level: a four-dimensional vector space over `T` ----

#[batch_impl_only(
    Module<Additive, Multiplicative> <T: Field<Additive, Multiplicative>> Quaternion<T>
        #Scalar{T}
        #scale{Quaternion::new(
            <T as Magma<Multiplicative>>::combine(s, v.w()),
            <T as Magma<Multiplicative>>::combine(s, v.x()),
            <T as Magma<Multiplicative>>::combine(s, v.y()),
            <T as Magma<Multiplicative>>::combine(s, v.z()),
        )},
)]
trait Module<Oa: Operator, Om: Operator>: AbelianGroup<Oa> {
    type Scalar;
    fn scale(s: &Self::Scalar, v: Self) -> Self;
}

// Marker levels: `batch_trait!`.

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

// ---- analytic layer: the euclidean norm of the four components ----

#[batch_impl_only(
    NormedSpace<Additive, Multiplicative> <T: Real + ClosedAdd + ClosedMul + Copy> Quaternion<T> #RealField{T}
        #norm_squared{
            *self.w() * *self.w() + *self.x() * *self.x() + *self.y() * *self.y() + *self.z() * *self.z()
        }
        #scale_real{
            Quaternion::new(*self.w() * r, *self.x() * r, *self.y() * r, *self.z() * r)
        },
)]
trait NormedSpace<Oa: Operator, Om: Operator>: VectorSpace<Oa, Om> {
    type RealField;
    fn norm_squared(&self) -> Self::RealField;
    fn scale_real(&self, r: Self::RealField) -> Self;
}

#[batch_impl_only(
    InnerSpace<Additive, Multiplicative> <T: Real + ClosedAdd + ClosedMul + Copy> Quaternion<T>
        #inner_product{*self.w() * *other.w() + *self.x() * *other.x() + *self.y() * *other.y() + *self.z() * *other.z()},
)]
trait InnerSpace<Oa: Operator, Om: Operator>: NormedSpace<Oa, Om> {
    fn inner_product(&self, other: &Self) -> Self::RealField;
}

#[batch_impl_only(
    FiniteDimVectorSpace<Additive, Multiplicative> <T: Real + ClosedAdd + ClosedMul + Copy> Quaternion<T>
        #dimension{4}
        #canonical_basis_element{
            match _i {
                0 => Quaternion::new(<T as Monoid<Multiplicative>>::identity(), <T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity()),
                1 => Quaternion::new(<T as Monoid<Additive>>::identity(), <T as Monoid<Multiplicative>>::identity(), <T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity()),
                2 => Quaternion::new(<T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity(), <T as Monoid<Multiplicative>>::identity(), <T as Monoid<Additive>>::identity()),
                3 => Quaternion::new(<T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity(), <T as Monoid<Multiplicative>>::identity()),
                _ => unreachable!(),
            }
        }
        #dot{self.inner_product(other)},
)]
trait FiniteDimVectorSpace<Oa: Operator, Om: Operator>: VectorSpace<Oa, Om> {
    fn dimension() -> usize;
    fn canonical_basis_element(_i: usize) -> Self;
    fn dot(&self, other: &Self) -> Self::Scalar;
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
