//! Lattices: meet/join semilattices and the full lattice.
//!
//! Unlike the tower traits, these are **not** operator-parameterized: meet
//! and join form a single partially-ordered operation (`min`/`max` on the
//! numerics), so one impl covers a type.

/// A meet-semilattice: a [`PartialOrd`] set with a greatest-lower-bound
/// operation.
pub trait MeetSemilattice: Sized + PartialOrd {
    /// The meet (greatest lower bound) of `self` and `other`.
    fn meet(&self, other: &Self) -> Self;
}

/// A join-semilattice: a [`PartialOrd`] set with a least-upper-bound
/// operation.
pub trait JoinSemilattice: Sized + PartialOrd {
    /// The join (least upper bound) of `self` and `other`.
    fn join(&self, other: &Self) -> Self;
}

/// A lattice: both semilattices, linked by the absorption laws
/// (`a ∧ (a ∨ b) == a`, `a ∨ (a ∧ b) == a` — tested in `crate::laws`).
pub trait Lattice: MeetSemilattice + JoinSemilattice {
    /// `(meet, join)` of `self` and `other` in one pass.
    fn meet_join(&self, other: &Self) -> (Self, Self) {
        (self.meet(other), self.join(other))
    }

    /// The smaller of the two, when comparable.
    fn partial_min<'a>(&'a self, other: &'a Self) -> Option<&'a Self> {
        if self <= other {
            Some(self)
        } else if other <= self {
            Some(other)
        } else {
            None
        }
    }

    /// The larger of the two, when comparable.
    fn partial_max<'a>(&'a self, other: &'a Self) -> Option<&'a Self> {
        if self >= other {
            Some(self)
        } else if other >= self {
            Some(other)
        } else {
            None
        }
    }
}

/// A bounded lattice: a [`Lattice`] with top and bottom elements.
pub trait BoundedLattice: Lattice {
    /// The greatest element.
    fn top() -> Self;

    /// The least element.
    fn bottom() -> Self;
}
