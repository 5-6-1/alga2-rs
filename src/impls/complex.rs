//! Complex impls: `Complex<T>` inherits the tower from `T`.
//!
//! Addition is component-wise; multiplication is `(ac−bd) + (ad+bc)i`
//! (needs `T`'s additive inverse, so the multiplicative side is defined over
//! rings); the field inverse is `conj(z)/|z|²`. The method bodies are
//! hand-written — a two-component algebra is clearer as plain `impl`s; only
//! the marker levels ride `batch_trait!`.

use batch_impl::batch_trait;

use crate::complex::Complex;
use crate::op::{Additive, Multiplicative};
use crate::tower::{
    AbelianGroup, CommutativeRing, ComplexField, DivisionRing, Field, FieldExtension, Group, Loop,
    Magma, Module, Monoid, Quasigroup, Real, Ring, Semigroup, Semiring, VectorSpace,
};

// ---- additive side: component-wise ----

impl<T: Magma<Additive>> Magma<Additive> for Complex<T> {
    fn combine(&self, rhs: &Self) -> Self {
        Complex::new(
            <T as Magma<Additive>>::combine(self.re(), rhs.re()),
            <T as Magma<Additive>>::combine(self.im(), rhs.im()),
        )
    }
}

impl<T: Monoid<Additive>> Monoid<Additive> for Complex<T> {
    fn identity() -> Self {
        Complex::new(<T as Monoid<Additive>>::identity(), <T as Monoid<Additive>>::identity())
    }
}

impl<T: Group<Additive>> Group<Additive> for Complex<T> {
    fn inverse(&self) -> Self {
        Complex::new(
            <T as Group<Additive>>::inverse(self.re()),
            <T as Group<Additive>>::inverse(self.im()),
        )
    }
}

// ---- multiplicative side: defined over rings (`ac−bd` needs the inverse) ----

impl<T: Ring<Additive, Multiplicative>> Magma<Multiplicative> for Complex<T> {
    fn combine(&self, rhs: &Self) -> Self {
        Complex::new(
            <T as Magma<Additive>>::combine(
                &<T as Magma<Multiplicative>>::combine(self.re(), rhs.re()),
                &<T as Group<Additive>>::inverse(&<T as Magma<Multiplicative>>::combine(
                    self.im(),
                    rhs.im(),
                )),
            ),
            <T as Magma<Additive>>::combine(
                &<T as Magma<Multiplicative>>::combine(self.re(), rhs.im()),
                &<T as Magma<Multiplicative>>::combine(self.im(), rhs.re()),
            ),
        )
    }
}

impl<T: Ring<Additive, Multiplicative>> Monoid<Multiplicative> for Complex<T> {
    fn identity() -> Self {
        Complex::new(<T as Monoid<Multiplicative>>::identity(), <T as Monoid<Additive>>::identity())
    }
}

// ---- division-ring inverse: conj(z) / |z|² ----

impl<T: Field<Additive, Multiplicative>> DivisionRing<Additive, Multiplicative> for Complex<T> {
    fn inv(&self) -> Self {
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
    }
}

// ---- module level: `Complex<T>` is a module over the real field `T` ----

impl<T: Field<Additive, Multiplicative>> Module<Additive, Multiplicative> for Complex<T> {
    type Scalar = T;

    fn scale(s: &Self::Scalar, v: Self) -> Self {
        Complex::new(
            <T as Magma<Multiplicative>>::combine(s, v.re()),
            <T as Magma<Multiplicative>>::combine(s, v.im()),
        )
    }
}

// ---- complex-field structure: `Complex<T>` over the real field `T` ----

impl<T: Real + Copy> ComplexField for Complex<T> {
    type RealField = T;

    fn from_real(re: Self::RealField) -> Self {
        Complex::new(re, <T as Monoid<Additive>>::identity())
    }

    fn re(&self) -> Self::RealField {
        *self.re()
    }

    fn im(&self) -> Self::RealField {
        *self.im()
    }

    fn conjugate(&self) -> Self {
        Complex::new(*self.re(), <T as Group<Additive>>::inverse(self.im()))
    }
}

// ---- field extension: `Complex<T>` is a degree-2 extension of `T` ----

impl<T: Real + Copy> FieldExtension<Additive, Multiplicative> for Complex<T> {
    type BaseField = T;

    fn degree() -> usize {
        2
    }

    fn trace(&self) -> Self::BaseField {
        <T as Magma<Additive>>::combine(self.re(), self.re())
    }

    fn norm(&self) -> Self::BaseField {
        <T as Magma<Additive>>::combine(
            &<T as Magma<Multiplicative>>::combine(self.re(), self.re()),
            &<T as Magma<Multiplicative>>::combine(self.im(), self.im()),
        )
    }
}

// ---- marker levels ----

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
