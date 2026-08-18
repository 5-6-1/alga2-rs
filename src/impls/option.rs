//! Option impls: the `T`-lifting monoid structure (bare-core, no alloc).
//!
//! `Option<T>` inherits `T`'s monoid levels by lifting the operation
//! component-wise: `None` is the additive identity and the absorbing element
//! under combination, `Some(x) ⊕ Some(y)` = `Some(x ⊕ y)`; under
//! `Multiplicative` the identity is `Some(T::identity())`.
//!
//! The ladder **stops at Monoid** for both operators: with `None` absorbing,
//! `Some(x)` has no inverse (the combine of two `Some`s is always `Some`),
//! so `Option` is never a group — and consequently never a ring (a ring's
//! additive part must be a group). This mirrors the "monoid with zero" role
//! of `Option` in algebra.

use batch_impl::batch_impl_only;

use crate::op::{Additive, Multiplicative};
use crate::tower::{Magma, Monoid, Semigroup};

#[batch_impl_only(
    [<T: Magma<Additive>> Magma<Additive>, <T: Magma<Multiplicative>> Magma<Multiplicative>]
        ^Option<T> #combine{match (self, rhs) { (Some(a), Some(b)) => Some(a.combine(b)), _ => None }},

)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    [<T: Semigroup<Additive>> Semigroup<Additive>,
    <T: Semigroup<Multiplicative>> Semigroup<Multiplicative>]^Option<T>,
)]
trait Semigroup<Op: Operator>: Magma<Op> {}

#[batch_impl_only(
    [<T: Monoid<Additive>> Monoid<Additive> #identity{None},
    <T: Monoid<Multiplicative>> Monoid<Multiplicative> #identity{Some(T::identity())}]^Option<T>
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
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
    fn option_additive_monoid() {
        // `None` is the identity and the absorbing element.
        let id = <Option<u8> as Monoid<Additive>>::identity();
        assert_eq!(id, None);
        assert_eq!(add(Some(3u8), Some(4)), Some(7));
        assert_eq!(add(Some(3), None), None);
        assert_eq!(add(None, Some(4)), None);
        // associativity
        assert_eq!(add(add(Some(3), Some(4)), Some(5)), add(Some(3), add(Some(4), Some(5))));
    }

    #[test]
    fn option_multiplicative_monoid() {
        // The multiplicative identity is `Some(1)`; `None` still absorbs.
        let one = <Option<u8> as Monoid<Multiplicative>>::identity();
        assert_eq!(one, Some(1));
        assert_eq!(mul(Some(3u8), Some(4)), Some(12));
        assert_eq!(mul(Some(3), None), None);
        assert_eq!(mul(None, Some(4)), None);
    }
}
