//! Container impls (`alloc`-gated): Vec / String / Box.
//!
//! `Vec<T>` and `String` are free monoids under concatenation (`T` needs no
//! algebraic structure, only `Clone` for `Vec`); `Box<T>` delegates every
//! level of `T` through one deref.

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use batch_impl::{batch_impl_only, batch_trait};

use crate::op::{Additive, Operator};
use crate::tower::{
    AbelianGroup, CommutativeRing, DivisionRing, Field, Group, Loop, Magma, Monoid, Quasigroup,
    Ring, Semigroup, Semiring,
};

// ---- Magma: Vec/String concatenate; smart pointers delegate ----

#[batch_impl_only(
    Magma<Additive> [
        <T: Clone> Vec<T> #combine{let mut v = self.clone(); v.extend(rhs.iter().cloned()); v},
        String #combine{let mut s = self.clone(); s.push_str(rhs); s},
    ],
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

// A smart pointer is a Magma for any operator its inner type is: pure
// delegation through one deref, so a single operator-generic spec covers
// every operator (Additive/Multiplicative) and every wrapper at once.

#[batch_impl_only(
    <Op: Operator> Magma<Op> [
        <T: Magma<Op>> Box<T> #combine{Box::new((**self).combine(&**rhs))},
        <T: Magma<Op>> Rc<T> #combine{Rc::new((**self).combine(&**rhs))},
        <T: Magma<Op>> Arc<T> #combine{Arc::new((**self).combine(&**rhs))},
    ],
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    Semigroup<Additive> [
        <T: Clone> Vec<T>,
        String,
    ],
)]
trait Semigroup<Op: Operator>: Magma<Op> {}

#[batch_impl_only(
    Monoid<Additive> [
        <T: Clone> Vec<T> #identity{Vec::new()},
        String #identity{String::new()},
    ],
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

#[batch_impl_only(
    <Op: Operator> Monoid<Op> [
        <T: Monoid<Op>> Box<T> #identity{Box::new(T::identity())},
        <T: Monoid<Op>> Rc<T> #identity{Rc::new(T::identity())},
        <T: Monoid<Op>> Arc<T> #identity{Arc::new(T::identity())},
    ],
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

// The additive ladder continues for smart pointers only (Vec/String are
// free monoids, no inverses); the multiplicative ladder stops at Monoid
// (integers have no multiplicative inverses).

// ---- semiring ladder: smart pointers delegate the ring/field levels ----
// Marker levels are one `batch_trait!` line per trait (the wrapper list in
// the target). `#blanket` is not applicable: every method-carrying level's
// methods return `Self`, which blanket delegation refuses (the forwarded
// call returns the inner type). Method levels hand-write the deref forward.

batch_trait! {
    Semigroup: <Op: Operator> Semigroup<Op> [
        <T: Semigroup<Op>> Box<T>,
        <T: Semigroup<Op>> Rc<T>,
        <T: Semigroup<Op>> Arc<T>,
    ];
    Quasigroup: <Op: Operator> Quasigroup<Op> [
        <T: Quasigroup<Op>> Box<T>,
        <T: Quasigroup<Op>> Rc<T>,
        <T: Quasigroup<Op>> Arc<T>,
    ];
    Loop: <Op: Operator> Loop<Op> [
        <T: Loop<Op>> Box<T>,
        <T: Loop<Op>> Rc<T>,
        <T: Loop<Op>> Arc<T>,
    ];
    Group: <Op: Operator> Group<Op> [
        <T: Group<Op>> Box<T> {fn inverse(&self) -> Self { Box::new((**self).inverse()) }},
        <T: Group<Op>> Rc<T> {fn inverse(&self) -> Self { Rc::new((**self).inverse()) }},
        <T: Group<Op>> Arc<T> {fn inverse(&self) -> Self { Arc::new((**self).inverse()) }},
    ];
    AbelianGroup: <Op: Operator> AbelianGroup<Op> [
        <T: AbelianGroup<Op>> Box<T>,
        <T: AbelianGroup<Op>> Rc<T>,
        <T: AbelianGroup<Op>> Arc<T>,
    ];
    Semiring: <Oa: Operator, Om: Operator> Semiring<Oa, Om> [
        <T: Semiring<Oa, Om>> Box<T>,
        <T: Semiring<Oa, Om>> Rc<T>,
        <T: Semiring<Oa, Om>> Arc<T>,
    ];
    Ring: <Oa: Operator, Om: Operator> Ring<Oa, Om> [
        <T: Ring<Oa, Om>> Box<T>,
        <T: Ring<Oa, Om>> Rc<T>,
        <T: Ring<Oa, Om>> Arc<T>,
    ];
    CommutativeRing: <Oa: Operator, Om: Operator> CommutativeRing<Oa, Om> [
        <T: CommutativeRing<Oa, Om>> Box<T>,
        <T: CommutativeRing<Oa, Om>> Rc<T>,
        <T: CommutativeRing<Oa, Om>> Arc<T>,
    ];
    Field: <Oa: Operator, Om: Operator> Field<Oa, Om> [
        <T: Field<Oa, Om>> Box<T>,
        <T: Field<Oa, Om>> Rc<T>,
        <T: Field<Oa, Om>> Arc<T>,
    ];
    DivisionRing: <Oa: Operator, Om: Operator> DivisionRing<Oa, Om> [
        <T: DivisionRing<Oa, Om>> Box<T> {fn inv(&self) -> Self { Box::new((**self).inv()) }},
        <T: DivisionRing<Oa, Om>> Rc<T> {fn inv(&self) -> Self { Rc::new((**self).inv()) }},
        <T: DivisionRing<Oa, Om>> Arc<T> {fn inv(&self) -> Self { Arc::new((**self).inv()) }},
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::op::Multiplicative;
    use crate::tower::{Magma, Monoid};
    use alloc::vec;

    fn add<T: Magma<Additive>>(a: T, b: T) -> T {
        <T as Magma<Additive>>::combine(&a, &b)
    }

    #[test]
    fn vec_is_free_monoid() {
        assert_eq!(add(vec![1u8, 2], vec![3]), vec![1, 2, 3]);
        assert_eq!(<Vec<u8> as Monoid<Additive>>::identity(), Vec::<u8>::new());
        assert_eq!(add(Vec::<u8>::new(), vec![3]), vec![3]);
    }

    #[test]
    fn string_is_free_monoid() {
        assert_eq!(add(String::from("ab"), String::from("cd")), "abcd");
        assert_eq!(<String as Monoid<Additive>>::identity(), String::new());
    }

    #[test]
    fn box_delegates() {
        assert_eq!(add(Box::new(3u8), Box::new(4)), Box::new(7));
        let one = <Box<u8> as Monoid<Additive>>::identity();
        assert_eq!(one, Box::new(0));
        // The whole ladder delegates: inverse and field inverse.
        let inv = <Box<i32> as Group<Additive>>::inverse(&Box::new(5));
        assert_eq!(inv, Box::new(-5));
        let finv = <Box<f64> as DivisionRing<Additive, Multiplicative>>::inv(&Box::new(2.0));
        assert_eq!(finv, Box::new(0.5));
    }

    #[test]
    fn rc_arc_delegate() {
        use crate::tower::{DivisionRing, Group};
        // Rc/Arc mirror Box: the whole ladder delegates through one deref.
        assert_eq!(add(Rc::new(3u8), Rc::new(4)), Rc::new(7));
        assert_eq!(add(Arc::new(3u8), Arc::new(4)), Arc::new(7));
        let one = <Rc<u8> as Monoid<Additive>>::identity();
        assert_eq!(one, Rc::new(0));
        let inv = <Rc<i32> as Group<Additive>>::inverse(&Rc::new(5));
        assert_eq!(inv, Rc::new(-5));
        let finv = <Arc<f64> as DivisionRing<Additive, Multiplicative>>::inv(&Arc::new(2.0));
        assert_eq!(finv, Arc::new(0.5));
    }
}
