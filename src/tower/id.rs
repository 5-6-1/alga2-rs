//! The identity-element wrapper.
//!
//! `Id<O>` denotes "the identity value under operator `O`" as a type — the
//! alga-style companion to [`Monoid::identity`](crate::tower::Monoid::identity)
//! for generic code that must name the identity without knowing the concrete
//! type's `identity()` implementation.

use core::marker::PhantomData;

use crate::op::Operator;

/// A unit type standing for the identity element under `O`.
pub struct Id<O: Operator = crate::op::Multiplicative>(PhantomData<O>);

impl<O: Operator> Default for Id<O> {
    fn default() -> Self {
        Id(PhantomData)
    }
}

impl<O: Operator> Clone for Id<O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O: Operator> Copy for Id<O> {}

impl<O: Operator> PartialEq for Id<O> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<O: Operator> Eq for Id<O> {}
