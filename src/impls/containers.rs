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
use batch_impl::batch_trait;

use crate::op::Operator;
use crate::tower::{
    AbelianGroup, CommutativeRing, DivisionRing, Field, Group, Loop, Magma, Monoid, Quasigroup,
    Ring, Semigroup, Semiring,
};

// ---- one batch_trait! block: Vec/String concatenate; smart pointers
// ---- delegate (the `impl{Box<T>}` shape template rewrites `Box::new` into
// ---- `Rc::new`/`Arc::new` per wrapper) ----

batch_trait! {
    @ptr=[Box,Rc,Arc];
    @impl=<T:@trait<> >@ptr T;
    Magma: <T: Clone> Vec<T>{
        fn combine(&self, rhs: &Self) -> Self {
            let mut v = self.clone();
            v.extend(rhs.iter().cloned());
            v
        }
    }, String{
        fn combine(&self, rhs: &Self) -> Self {
            let mut s = self.clone();
            s.push_str(rhs);
            s
        }
    },
        <Op: Operator> @trait<Op> @impl impl{Box<_>}{
        fn combine(&self, rhs: &Self) -> Self {
            Box::new((**self).combine(&**rhs))
        }
    };
    Semigroup: <T: Clone> Vec<T>, String,
        <Op: Operator> @trait<Op> @impl;
    Monoid: <T: Clone> Vec<T>{
        fn identity() -> Self { Vec::new() }
    },
        String{
        fn identity() -> Self { String::new() }
    },
        <Op: Operator> @trait<Op> <T: Monoid<>> @ptr T impl{Box<_>}{
        fn identity() -> Self { Box::new(T::identity()) }
    };
    Quasigroup: <Op: Operator> @trait<Op> @impl;
    Loop: <Op: Operator> @trait<Op> @impl;
    Group: <Op: Operator> @trait<Op> @impl impl{Box<_>} {
        fn inverse(&self) -> Self { Box::new((**self).inverse()) }
    };
    AbelianGroup: <Op: Operator> @trait<Op> @impl;
    Semiring: <Oa: Operator, Om: Operator> @trait<Oa, Om> @impl;
    Ring: <Oa: Operator, Om: Operator> @trait<Oa, Om> @impl;
    CommutativeRing: <Oa: Operator, Om: Operator> @trait<Oa, Om> @impl;
    Field: <Oa: Operator, Om: Operator> @trait<Oa, Om> @impl;
    DivisionRing: <Oa: Operator, Om: Operator> @trait<Oa, Om> @impl impl{Box<_>} {
        fn inv(&self) -> Self { Box::new((**self).inv()) }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::op::Additive;
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
