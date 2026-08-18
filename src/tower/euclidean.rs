//! Euclidean domains.
//!
//! An integral domain with a euclidean division: every `a, b` (b ≠ 0) splits
//! as `a = q·b + r` with the remainder's euclidean norm smaller than the
//! divisor's — the integers are the archetype, and the gcd/euclidean
//! algorithm is the standard inhabitant.

use crate::op::Operator;

use super::{CommutativeRing, Monoid};

/// An integral domain with a euclidean division.
pub trait EuclideanDomain<Oa: Operator, Om: Operator>: CommutativeRing<Oa, Om> {
    /// The quotient and remainder of the euclidean division `self ÷ divisor`.
    fn quot_rem(&self, divisor: &Self) -> (Self, Self)
    where
        Self: Sized;

    /// The euclidean norm (a nonnegative size measure, `0` only for zero).
    fn euclidean_norm(&self) -> u128;

    /// The quotient of the euclidean division.
    fn quot(&self, divisor: &Self) -> Self
    where
        Self: Sized,
    {
        self.quot_rem(divisor).0
    }

    /// The remainder of the euclidean division.
    fn rem(&self, divisor: &Self) -> Self
    where
        Self: Sized,
    {
        self.quot_rem(divisor).1
    }

    /// The greatest common divisor via the euclidean algorithm.
    fn gcd(&self, other: &Self) -> Self
    where
        Self: PartialEq + Clone + Monoid<Oa> + Sized,
    {
        let zero = <Self as Monoid<Oa>>::identity();
        let mut a = self.clone();
        let mut b = other.clone();
        while b != zero {
            let (_, r) = a.quot_rem(&b);
            a = b;
            b = r;
        }
        a
    }
}
