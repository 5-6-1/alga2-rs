//! Integer powers in a monoid.
//!
//! Any monoid supports `aⁿ` by square-and-multiply — the default
//! implementation here runs in `O(log n)` combines and needs only the
//! monoid structure.

use crate::op::Operator;

use super::{Magma, Monoid};

/// Integer powers in a [`Monoid`] via square-and-multiply.
pub trait Power<Op: Operator>: Monoid<Op> + Clone {
    /// `selfⁿ` (`n == 0` yields the identity).
    fn pow(&self, n: u32) -> Self {
        let mut acc = Self::identity();
        let mut base = self.clone();
        let mut e = n;
        while e > 0 {
            if e & 1 == 1 {
                acc = <Self as Magma<Op>>::combine(&acc, &base);
            }
            base = <Self as Magma<Op>>::combine(&base, &base);
            e >>= 1;
        }
        acc
    }
}
