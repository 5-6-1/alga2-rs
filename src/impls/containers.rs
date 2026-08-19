//! Container impls (`alloc`-gated): Vec / String / Box.
//!
//! `Vec<T>` and `String` are free monoids under concatenation (`T` needs no
//! algebraic structure, only `Clone` for `Vec`); `Box<T>` delegates every
//! level of `T` through one deref.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use batch_impl::{batch_impl_only, batch_trait};

use crate::op::{Additive, Multiplicative};
use crate::tower::{
    AbelianGroup, CommutativeRing, DivisionRing, Field, Group, Loop, Magma, Monoid, Quasigroup,
    Ring, Semigroup, Semiring,
};

// ---- Magma: Vec/String concatenate, Box delegates ----

#[batch_impl_only(
    Magma<Additive> [
        <T: Clone> Vec<T> #combine{let mut v = self.clone(); v.extend(rhs.iter().cloned()); v},
        String #combine{let mut s = self.clone(); s.push_str(rhs); s},
        <T: Magma<> > Box<T> #combine{Box::new((**self).combine(&**rhs))},
    ],
    Magma<Multiplicative> <T: Magma<> > Box<T> #combine{Box::new((**self).combine(&**rhs))},
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    Semigroup<Additive>[
        <T: Clone> Vec<T>,
        String,
        <T: Semigroup<> > Box<T>,
    ],
    Semigroup<Multiplicative> <T: Semigroup<> > Box<T>,
)]
trait Semigroup<Op: Operator>: Magma<Op> {}

#[batch_impl_only(
    Monoid<Additive> [
        <T: Clone> Vec<T> #identity{Vec::new()},
        String #identity{String::new()},
        <T: Monoid<> > Box<T> #identity{Box::new(T::identity())},
    ],
    Monoid<Multiplicative> <T: Monoid<> > Box<T> #identity{Box::new(T::identity())},
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

// The additive ladder continues for Box only (Vec/String are free monoids,
// no inverses); the multiplicative ladder stops at Monoid for Box.

// ---- semiring ladder: Box delegates the ring/field levels ----

batch_trait! {
    Quasigroup:Quasigroup<Additive> <T: Quasigroup<> > Box<T>;
    Loop:Loop<Additive> <T: Loop<> > Box<T>;
    Group:Group<Additive> <T: Group<> > Box<T>
        {fn inverse(&self) -> Self {
            Box::new((**self).inverse())
        }};
    AbelianGroup:AbelianGroup<Additive> <T: AbelianGroup<> > Box<T>;
    Semiring:Semiring<Additive, Multiplicative> <T: Semiring<> > Box<T>;
    Ring:Ring<Additive, Multiplicative> <T: Ring<> > Box<T>;
    CommutativeRing:CommutativeRing<Additive, Multiplicative> <T: CommutativeRing<> > Box<T>;
    Field:Field<Additive, Multiplicative> <T: Field<> > Box<T>;
    DivisionRing:DivisionRing<Additive, Multiplicative> <T: DivisionRing<> > Box<T>
        {fn inv(&self) -> Self {
            Box::new((**self).inv())
        }},
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
