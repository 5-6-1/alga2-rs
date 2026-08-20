//! Map/set impls (`std`-gated): HashMap / BTreeMap / HashSet / BTreeSet free
//! monoids.
//!
//! All four combine by (right-biased) union: the maps keep the right-hand
//! value for colliding keys, which keeps the operation associative.

use batch_impl::{batch_impl_only, batch_trait};

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

use crate::op::Additive;
use crate::tower::{Magma, Monoid, Semigroup};

#[batch_impl_only(
    Magma<Additive> [
        <K: Clone + Eq + Hash, V: Clone> HashMap<K, V> #combine{
            let mut m = self.clone();
            for (k, v) in rhs {
                m.insert(k.clone(), v.clone());
            }
            m
        },
        <K: Clone + Eq + Hash> HashSet<K> #combine{
            let mut s = self.clone();
            for x in rhs {
                s.insert(x.clone());
            }
            s
        },
        <K: Clone + Ord, V: Clone> BTreeMap<K, V> #combine{
            let mut m = self.clone();
            for (k, v) in rhs {
                m.insert(k.clone(), v.clone());
            }
            m
        },
        <K: Clone + Ord> BTreeSet<K> #combine{
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
    Monoid<Additive> [
        <K: Clone + Eq + Hash, V: Clone> HashMap<K, V> #identity{HashMap::new()},
        <K: Clone + Eq + Hash> HashSet<K> #identity{HashSet::new()},
        <K: Clone + Ord, V: Clone> BTreeMap<K, V> #identity{BTreeMap::new()},
        <K: Clone + Ord> BTreeSet<K> #identity{BTreeSet::new()},
    ]
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

batch_trait! {
    Semigroup: Semigroup<Additive> [
        <K: Clone + Eq + Hash, V: Clone> HashMap<K, V>,
        <K: Clone + Eq + Hash> HashSet<K>,
        <K: Clone + Ord, V: Clone> BTreeMap<K, V>,
        <K: Clone + Ord> BTreeSet<K>,
    ];
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

    #[test]
    fn btree_collections_union() {
        let m1 = BTreeMap::from([(1u8, 10u8), (2, 20)]);
        let m2 = BTreeMap::from([(2, 200), (3, 30)]);
        let joined = add(m1, m2);
        assert_eq!(joined.get(&2), Some(&200));
        assert_eq!(joined.len(), 3);
        let s1 = BTreeSet::from([1u8, 2]);
        let s2 = BTreeSet::from([2, 3]);
        assert_eq!(add(s1, s2), BTreeSet::from([1, 2, 3]));
        assert_eq!(<BTreeSet<u8> as Monoid<Additive>>::identity(), BTreeSet::new());
    }
}
