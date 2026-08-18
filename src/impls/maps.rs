//! Map/set impls (`std`-gated): HashMap / HashSet free monoids.
//!
//! Both combine by (right-biased) union: `HashMap` keeps the right-hand
//! value for colliding keys, which keeps the operation associative.

use batch_impl::batch_impl_only;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::op::Additive;

#[batch_impl_only(
    #crate::tower::Magma:
    Magma<Additive> <K: Clone + Eq + Hash> [
        <V: Clone> HashMap<K, V> #combine{
            let mut m = self.clone();
            for (k, v) in rhs {
                m.insert(k.clone(), v.clone());
            }
            m
        },
        HashSet<K> #combine{
            let mut s = self.clone();
            for x in rhs {
                s.insert(x.clone());
            }
            s
        }
    ]
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    #crate::tower::Semigroup:
    Semigroup<Additive> <K: Clone + Eq + Hash> [
        <V: Clone> HashMap<K, V>,
        HashSet<K>,
    ]
)]
trait Semigroup<Op: Operator>: Magma<Op> {}

#[batch_impl_only(
    #crate::tower::Monoid:
    Monoid<Additive> <K: Clone + Eq + Hash> [
        <V: Clone> HashMap<K, V> #identity{HashMap::new()},
        HashSet<K> #identity{HashSet::new()},
    ]
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::{Magma, Monoid};

    fn add<T: Magma<Additive>>(a: T, b: T) -> T {
        <T as Magma<Additive>>::combine(&a, &b)
    }

    #[test]
    fn hashmap_right_biased_union() {
        let m1 = HashMap::from([(1u8, 10u8), (2, 20)]);
        let m2 = HashMap::from([(2, 200), (3, 30)]);
        let joined = add(m1, m2);
        assert_eq!(joined.len(), 3);
        assert_eq!(joined.get(&1), Some(&10));
        // Collision resolves to the right-hand value (keeps associativity).
        assert_eq!(joined.get(&2), Some(&200));
        assert_eq!(joined.get(&3), Some(&30));
        assert_eq!(<HashMap<u8, u8> as Monoid<Additive>>::identity(), HashMap::new());
    }

    #[test]
    fn hashset_union() {
        let s1 = HashSet::from([1u8, 2]);
        let s2 = HashSet::from([2, 3]);
        assert_eq!(add(s1, s2), HashSet::from([1, 2, 3]));
        assert_eq!(<HashSet<u8> as Monoid<Additive>>::identity(), HashSet::new());
    }
}
