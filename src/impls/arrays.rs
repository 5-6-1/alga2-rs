//! Array impls: `[T; N]` is component-wise, like tuples, but without the
//! std arity ceiling — arrays of any `N` carry `Clone`/`PartialEq`/etc., so
//! the whole tower runs on `[T; N]` (contrast the tuple caps in `tuples.rs`).

use batch_impl::{batch_impl_only, batch_trait};

use crate::op::{Additive, Multiplicative};
use crate::tower::{
    AbelianGroup, CommutativeRing, Field, FreeModule, Group, Loop, Magma, Module, Monoid,
    Quasigroup, Ring, Semigroup, Semiring, VectorSpace,
};

#[batch_impl_only(
    Magma<Additive> <T: Magma<Additive>, const N: usize> [T; N]
        #combine{core::array::from_fn(|i| <T as Magma<Additive>>::combine(&self[i], &rhs[i]))},
    Magma<Multiplicative> <T: Magma<Multiplicative>, const N: usize> [T; N]
        #combine{core::array::from_fn(|i| <T as Magma<Multiplicative>>::combine(&self[i], &rhs[i]))},
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    Monoid<Additive> <T: Monoid<Additive>, const N: usize> [T; N]
        #identity{core::array::from_fn(|_| <T as Monoid<Additive>>::identity())},
    Monoid<Multiplicative> <T: Monoid<Multiplicative>, const N: usize> [T; N]
        #identity{core::array::from_fn(|_| <T as Monoid<Multiplicative>>::identity())},
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

#[batch_impl_only(
    Group<Additive> <T: Group<Additive>, const N: usize> [T; N]
        #inverse{core::array::from_fn(|i| <T as Group<Additive>>::inverse(&self[i]))},
)]
trait Group<Op: Operator>: Loop<Op> {
    fn inverse(&self) -> Self;
}

#[batch_impl_only(
    Module<Additive, Multiplicative> <T: Module<Additive, Multiplicative> + Copy, const N: usize> [T; N]
        #Scalar{<T as Module<Additive, Multiplicative>>::Scalar}
        #scale{core::array::from_fn(|i| <T as Module<Additive, Multiplicative>>::scale(s, v[i]))},
)]
trait Module<Oa: Operator, Om: Operator>: AbelianGroup<Oa> {
    type Scalar;
    fn scale(s: &Self::Scalar, v: Self) -> Self;
}

// Marker levels: one `batch_trait!` segment per trait (all components must
// carry the level).

batch_trait! {
    Semigroup: Semigroup<Additive> <T: Semigroup<Additive>, const N: usize> [T; N],
        Semigroup<Multiplicative> <T: Semigroup<Multiplicative>, const N: usize> [T; N];
    Quasigroup: Quasigroup<Additive> <T: Quasigroup<Additive>, const N: usize> [T; N];
    Loop: Loop<Additive> <T: Loop<Additive>, const N: usize> [T; N];
    AbelianGroup: AbelianGroup<Additive> <T: AbelianGroup<Additive>, const N: usize> [T; N];
    Semiring: Semiring<Additive, Multiplicative> <T: Semiring<Additive, Multiplicative>, const N: usize> [T; N];
    Ring: Ring<Additive, Multiplicative> <T: Ring<Additive, Multiplicative>, const N: usize> [T; N];
    CommutativeRing: CommutativeRing<Additive, Multiplicative> <T: CommutativeRing<Additive, Multiplicative>, const N: usize> [T; N];
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
}
