//! Map/set impls (`std`-gated): HashMap / BTreeMap / HashSet / BTreeSet free
//! monoids.
//!
//! All four combine by (right-biased) union: the maps keep the right-hand
//! value for colliding keys, which keeps the operation associative.

use batch_impl::batch_trait;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

use crate::op::Additive;
use crate::tower::{Magma, Monoid, Semigroup};

batch_trait! {
    Magma: @trait<Additive> <K: Clone>[
        <V:Clone>{
            fn combine(&self, rhs: &Self) -> Self {
                let mut m = self.clone();
                for (k, v) in rhs { m.insert(k.clone(), v.clone()); }
                m
            }
        }[<K:Eq+Hash>HashMap<K, V> ,<K:Ord>BTreeMap<K, V>],
        {
            fn combine(&self, rhs: &Self) -> Self {
                let mut s = self.clone();
                for x in rhs { s.insert(x.clone()); }
                s
            }
        }[<K:Eq+Hash>HashSet<K> ,<K:Ord>BTreeSet<K>]
    ];
    Semigroup: @trait<Additive> <K:Clone> [
        <V:Clone> [<K:Eq+Hash>HashMap<K,V>,<K:Ord>BTreeMap<K,V>],
        <K:Eq+Hash>HashSet<K>,<K:Ord>BTreeSet<K>,
    ];
    Monoid: @trait<Additive> <K:Clone> [
        <V:Clone> [<K:Eq+Hash>HashMap<K,V>,<K:Ord>BTreeMap<K,V>] impl{T<_,_>},
        [<K:Eq+Hash>HashSet<K>,<K:Ord>BTreeSet<K>]impl{T<_>}
    ]{
        fn identity() -> Self {
            T::new()
        }
    };
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
