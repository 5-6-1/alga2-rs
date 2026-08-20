//! Array impls: `[T; N]` is component-wise, like tuples, but without the
//! std arity ceiling — arrays of any `N` carry `Clone`/`PartialEq`/etc., so
//! the whole tower runs on `[T; N]` (contrast the tuple caps in `tuples.rs`).
//! A `[C; N]` array is also a polynomial over `C` (fixed-length dense form;
//! the degree scan is hand-written — too bulky for a directive body).

use batch_impl::batch_trait;

use crate::op::{Additive, Multiplicative};
use crate::tower::{
    AbelianGroup, CommutativeRing, Field, FreeModule, Group, Loop, Magma, Module, Monoid,
    Polynomial, Quasigroup, Ring, Semigroup, Semiring, VectorSpace,
};

// ---- polynomials: a fixed-length coefficient array (hand-written) ----

impl<C: Ring<Additive, Multiplicative> + PartialEq + Clone, const N: usize> Polynomial for [C; N] {
    type Coefficient = C;

    fn degree(&self) -> usize {
        let zero = <C as Monoid<Additive>>::identity();
        let mut d = 0usize;
        for i in (0..N).rev() {
            if self[i] != zero {
                d = i;
                break;
            }
        }
        d
    }

    fn coefficient(&self, i: usize) -> Self::Coefficient {
        self[i].clone()
    }
}

// ---- one batch_trait! block: the component-wise tower (the `from_fn`
// ---- bodies are one line each; the markers are plain lists) ----

batch_trait! {
    Magma: Magma<Additive> <T: Magma<Additive>, const N: usize> [T; N]
        {fn combine(&self, rhs: &Self) -> Self {
            core::array::from_fn(|i| <T as Magma<Additive>>::combine(&self[i], &rhs[i]))
        }},
        Magma<Multiplicative> <T: Magma<Multiplicative>, const N: usize> [T; N]
        {fn combine(&self, rhs: &Self) -> Self {
            core::array::from_fn(|i| <T as Magma<Multiplicative>>::combine(&self[i], &rhs[i]))
        }};
    Semigroup: Semigroup<Additive> <T: Semigroup<Additive>, const N: usize> [T; N],
        Semigroup<Multiplicative> <T: Semigroup<Multiplicative>, const N: usize> [T; N];
    Monoid: Monoid<Additive> <T: Monoid<Additive>, const N: usize> [T; N]
        {fn identity() -> Self { core::array::from_fn(|_| <T as Monoid<Additive>>::identity()) }},
        Monoid<Multiplicative> <T: Monoid<Multiplicative>, const N: usize> [T; N]
        {fn identity() -> Self { core::array::from_fn(|_| <T as Monoid<Multiplicative>>::identity()) }};
    Quasigroup: Quasigroup<Additive> <T: Quasigroup<Additive>, const N: usize> [T; N];
    Loop: Loop<Additive> <T: Loop<Additive>, const N: usize> [T; N];
    Group: Group<Additive> <T: Group<Additive>, const N: usize> [T; N]
        {fn inverse(&self) -> Self {
            core::array::from_fn(|i| <T as Group<Additive>>::inverse(&self[i]))
        }};
    AbelianGroup: AbelianGroup<Additive> <T: AbelianGroup<Additive>, const N: usize> [T; N];
    Semiring: Semiring<Additive, Multiplicative> <T: Semiring<Additive, Multiplicative>, const N: usize> [T; N];
    Ring: Ring<Additive, Multiplicative> <T: Ring<Additive, Multiplicative>, const N: usize> [T; N];
    CommutativeRing: CommutativeRing<Additive, Multiplicative> <T: CommutativeRing<Additive, Multiplicative>, const N: usize> [T; N];
    Module: Module<Additive, Multiplicative> <T: Module<Additive, Multiplicative> + Copy, const N: usize> [T; N]
        {type Scalar = <T as Module<Additive, Multiplicative>>::Scalar;
         fn scale(s: &Self::Scalar, v: Self) -> Self {
            core::array::from_fn(|i| <T as Module<Additive, Multiplicative>>::scale(s, v[i]))
        }};
    VectorSpace: VectorSpace<Additive, Multiplicative> <T: VectorSpace<Additive, Multiplicative> + Copy, const N: usize> [T; N]
        where{Self::Scalar: Field<Additive, Multiplicative>};
    // `[T; N]` is the free module R^N when the components are scalars
    // (`T` itself a monoid under both operators — the numerics, `ModN`).
    FreeModule: FreeModule<Additive, Multiplicative> <T: FreeModule<Additive, Multiplicative> + Monoid<Additive> + Monoid<Multiplicative> + Copy, const N: usize> [T; N]
        {fn rank() -> usize { N } fn basis_element(_i: usize) -> Self {
            core::array::from_fn(|i| if i == _i {
                <T as Monoid<Multiplicative>>::identity()
            } else {
                <T as Monoid<Additive>>::identity()
            })
        } fn coordinate(&self, i: usize) -> Self::Scalar {
            <T as FreeModule<Additive, Multiplicative>>::coordinate(&self[i], 0)
        }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::{Magma, Monoid};

    #[test]
    fn arrays_are_componentwise() {
        let a = [1u8, 2, 3];
        let b = [4u8, 5, 6];
        let s = <[u8; 3] as Magma<Additive>>::combine(&a, &b);
        assert_eq!(s, [5, 7, 9]);
        let m = <[u8; 3] as Magma<Multiplicative>>::combine(&a, &b);
        assert_eq!(m, [4, 10, 18]);
        let zero = <[u8; 3] as Monoid<Additive>>::identity();
        assert_eq!(zero, [0, 0, 0]);
        // Arrays have no std arity ceiling: 20 elements work.
        let a20 = [1u8; 20];
        let b20 = [2u8; 20];
        let s20 = <[u8; 20] as Magma<Additive>>::combine(&a20, &b20);
        assert_eq!(s20, [3u8; 20]);
    }

    #[test]
    fn arrays_are_polynomials() {
        use crate::tower::Polynomial;
        // 2 + 3x + 0x² + 4x³ → degree 3.
        let p = [2u8, 3, 0, 4];
        assert_eq!(p.degree(), 3);
        assert_eq!(p.coefficient(0), 2);
        assert_eq!(p.coefficient(3), 4);
        // The zero polynomial has degree 0.
        let z = [0u8, 0, 0, 0];
        assert_eq!(z.degree(), 0);
    }
}
