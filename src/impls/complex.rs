//! Complex impls: `Complex<T>` inherits the tower from `T`.
//!
//! Addition is component-wise; multiplication is `(ac−bd) + (ad+bc)i`
//! (needs `T`'s additive inverse, so the multiplicative side is defined over
//! rings); the field inverse is `conj(z)/|z|²`.

use batch_impl::{batch_impl_only, batch_trait};

use crate::complex::Complex;
use crate::op::{Additive, Multiplicative};
use crate::tower::{
    AbelianGroup, CommutativeRing, DivisionRing, Field, FieldExtension, Group, Loop, Magma, Module,
    Monoid, Quasigroup, Real, Ring, Semigroup, Semiring, VectorSpace,
};

// ---- additive side: component-wise ----

#[batch_impl_only(
    Magma<Additive> <T: @trait<> > Complex<T> impl{@trait<>} #combine{
        Complex::new(
            <T as Magma<> >::combine(self.re(), rhs.re()),
            <T as Magma<> >::combine(self.im(), rhs.im()),
        )
    },
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    Monoid<Additive> <T: @trait<> > Complex<T> impl{@trait<>} #identity{
        Complex::new(
            <T as Monoid<> >::identity(),
            <T as Monoid<> >::identity(),
        )
    },
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

#[batch_impl_only(
    Group<Additive> <T: @trait<>> Complex<T> impl{@trait<>} #inverse{
        Complex::new(
            <T as Group<> >::inverse(self.re()),
            <T as Group<> >::inverse(self.im()),
        )
    },
)]
trait Group<Op: Operator>: Loop<Op> {
    fn inverse(&self) -> Self;
}

// ---- multiplicative side: defined over rings (`ac−bd` needs the inverse) ----

#[batch_impl_only(
    Magma<Multiplicative> <T: Ring<Additive, Multiplicative>> Complex<T> impl{@trait<>} #combine{
        Complex::new(
            <T as Magma<Additive>>::combine(
                &<T as Magma<> >::combine(self.re(), rhs.re()),
                &<T as Group<Additive>>::inverse(&<T as Magma<> >::combine(
                    self.im(),
                    rhs.im(),
                )),
            ),
            <T as Magma<Additive>>::combine(
                &<T as Magma<> >::combine(self.re(), rhs.im()),
                &<T as Magma<> >::combine(self.im(), rhs.re()),
            ),
        )
    },
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    Monoid<Multiplicative> <T: Ring<Additive, Multiplicative>> Complex<T> #identity{
        Complex::new(
            <T as Monoid<Multiplicative>>::identity(),
            <T as Monoid<Additive>>::identity(),
        )
    },
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

// ---- semiring ladder: `Complex<T>` is a ring/field when `T` is ----

// The multiplicative side is defined over rings (see above), so
// `Complex<T>: Semiring` requires `T: Ring` too — the multiplicative monoid
// exists only there; `Complex<T>` is then actually a ring.

#[batch_impl_only(
    DivisionRing<Additive, Multiplicative> <T: Field<Additive, Multiplicative>> Complex<T> #inv{
        let d = <T as Magma<Additive>>::combine(
            &<T as Magma<Multiplicative>>::combine(self.re(), self.re()),
            &<T as Magma<Multiplicative>>::combine(self.im(), self.im()),
        );
        let inv_d = <T as DivisionRing<Additive, Multiplicative>>::inv(&d);
        Complex::new(
            <T as Magma<Multiplicative>>::combine(self.re(), &inv_d),
            <T as Magma<Multiplicative>>::combine(
                &<T as Group<Additive>>::inverse(self.im()),
                &inv_d,
            ),
        )
    },
)]
trait DivisionRing<Oa: Operator, Om: Operator>: Ring<Oa, Om> {
    fn inv(&self) -> Self;
}

// ---- module level: `Complex<T>` is a module over the real field `T` ----

#[batch_impl_only(
    Module<Additive, Multiplicative> <T: Field<Additive, Multiplicative>> Complex<T>
        #Scalar{T}
        #scale{Complex::new(
            <T as Magma<Multiplicative>>::combine(s, v.re()),
            <T as Magma<Multiplicative>>::combine(s, v.im()),
        )},
)]
trait Module<Oa: Operator, Om: Operator>: AbelianGroup<Oa> {
    type Scalar;
    fn scale(s: &Self::Scalar, v: Self) -> Self;
}

// Marker levels (no directives, no duplicated signatures): `batch_trait!`.

batch_trait! {
    Semigroup: Semigroup<Additive> <T: @trait<>> Complex<T>,
        Semigroup<Multiplicative> <T: Ring<Additive, Multiplicative>> Complex<T>;
    Quasigroup: Quasigroup<Additive> <T: @trait<>> Complex<T>;
    Loop: Loop<Additive> <T: @trait<>> Complex<T>;
    AbelianGroup: AbelianGroup<Additive> <T: @trait<>> Complex<T>;
    Semiring: Semiring<Additive, Multiplicative> <T: Ring<Additive, Multiplicative>> Complex<T>;
    Ring: Ring<Additive, Multiplicative> <T: @trait<>> Complex<T>;
    CommutativeRing: CommutativeRing<Additive, Multiplicative> <T: @trait<>> Complex<T>;
    Field: Field<Additive, Multiplicative> <T: @trait<>> Complex<T>;
    VectorSpace: VectorSpace<Additive, Multiplicative> <T: Field<Additive, Multiplicative>> Complex<T>
        where{Self::Scalar: Field<Additive, Multiplicative>};
}

// ---- complex-field structure: `Complex<T>` over the real field `T` ----

#[batch_impl_only(
    #crate::tower::ComplexField:
    <T: Real + Copy> Complex<T>
        #RealField{T}
        #from_real{Complex::new(re, <T as Monoid<Additive>>::identity())}
        #re{*self.re()}
        #im{*self.im()}
        #conjugate{Complex::new(*self.re(), <T as Group<Additive>>::inverse(self.im()))},
)]
trait ComplexField: Field<Additive, Multiplicative> {
    type RealField;
    fn from_real(re: Self::RealField) -> Self;
    fn re(&self) -> Self::RealField;
    fn im(&self) -> Self::RealField;
    fn conjugate(&self) -> Self;
}

// ---- field extension: `Complex<T>` is a degree-2 extension of `T` ----

#[batch_impl_only(
    FieldExtension<Additive, Multiplicative> <T: Real + Copy> Complex<T>
        #BaseField{T}
        #degree{2}
        #trace{<T as Magma<Additive>>::combine(self.re(), self.re())}
        #norm{<T as Magma<Additive>>::combine(
            &<T as Magma<Multiplicative>>::combine(self.re(), self.re()),
            &<T as Magma<Multiplicative>>::combine(self.im(), self.im()),
        )},
)]
trait FieldExtension<Oa: Operator, Om: Operator>:
    Field<Oa, Om> + VectorSpace<Oa, Om, Scalar = <Self as FieldExtension<Oa, Om>>::BaseField>
{
    type BaseField;
    fn degree() -> usize;
    fn trace(&self) -> Self::BaseField;
    fn norm(&self) -> Self::BaseField;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::Magma;

    fn add<T: Magma<Additive>>(a: T, b: T) -> T {
        <T as Magma<Additive>>::combine(&a, &b)
    }

    fn mul<T: Magma<Multiplicative>>(a: T, b: T) -> T {
        <T as Magma<Multiplicative>>::combine(&a, &b)
    }

    #[test]
    fn complex_add() {
        let z = add(Complex::new(1i32, 2), Complex::new(3, 4));
        assert_eq!(z.re(), &4);
        assert_eq!(z.im(), &6);
    }

    #[test]
    fn complex_mul() {
        // (1+2i)(3+4i) = (3−8) + (4+6)i = −5 + 10i
        let z = mul(Complex::new(1i32, 2), Complex::new(3, 4));
        assert_eq!(z.re(), &-5);
        assert_eq!(z.im(), &10);
        // 1+0i is the multiplicative identity.
        let one = <Complex<i32> as Monoid<Multiplicative>>::identity();
        assert_eq!(mul(Complex::new(3, 4), one).re(), &3);
        assert_eq!(mul(Complex::new(3, 4), one).im(), &4);
    }

    #[test]
    fn complex_field_inverse() {
        // (1+0i)⁻¹ = 1+0i; (0+1i)⁻¹ = −i
        let z = Complex::new(1.0f64, 0.0);
        let inv = <Complex<f64> as DivisionRing<Additive, Multiplicative>>::inv(&z);
        assert_eq!(inv.re(), &1.0);
        assert_eq!(inv.im(), &0.0);
        let z = Complex::new(0.0, 1.0);
        let inv = <Complex<f64> as DivisionRing<Additive, Multiplicative>>::inv(&z);
        assert_eq!(inv.re(), &0.0);
        assert_eq!(inv.im(), &-1.0);
    }

    #[test]
    fn complex_field_structure() {
        use crate::tower::ComplexField;
        let z = Complex::new(3.0f64, 4.0);
        assert_eq!(z.re(), &3.0);
        assert_eq!(z.im(), &4.0);
        assert_eq!(z.conjugate(), Complex::new(3.0, -4.0));
        assert_eq!(<Complex<f64> as ComplexField>::from_real(2.5), Complex::new(2.5, 0.0));
        // C is a degree-2 extension of R.
        use crate::tower::FieldExtension;
        assert_eq!(<Complex<f64> as FieldExtension<Additive, Multiplicative>>::degree(), 2);
        assert_eq!(z.trace(), 6.0);
        assert_eq!(z.norm(), 25.0);
    }
}
