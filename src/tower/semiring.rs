//! The multiplicative ladder: Semiring → Ring → CommutativeRing → Field.
//!
//! These traits combine both operators: the additive part is the abelian
//! ladder from the parent module, the multiplicative part is the monoid
//! under [`Multiplicative`](crate::op::Multiplicative), and the two are tied
//! together by distributivity — a law, tested in `crate::laws`. `Field`
//! additionally requires `0 != 1` (a law).

use crate::op::Operator;

use super::{AbelianGroup, Monoid, VectorSpace};

/// A semiring: additive [`Monoid`] + multiplicative [`Monoid`] + distributivity.
pub trait Semiring<Oa: Operator, Om: Operator>: Monoid<Oa> + Monoid<Om> {}

/// A ring: a [`Semiring`] whose additive part is an abelian group
/// (the additively-commutative law is part of the definition).
pub trait Ring<Oa: Operator, Om: Operator>: Semiring<Oa, Om> + AbelianGroup<Oa> {}

/// A commutative ring: a [`Ring`] whose multiplicative part commutes.
pub trait CommutativeRing<Oa: Operator, Om: Operator>: Ring<Oa, Om> {}

/// A division ring: a [`Ring`] where every nonzero element has a
/// multiplicative inverse (not necessarily commutative). Every field is a
/// division ring; the quaternions are the classic non-commutative example.
pub trait DivisionRing<Oa: Operator, Om: Operator>: Ring<Oa, Om> {
    /// The multiplicative inverse of `self`.
    fn inv(&self) -> Self;
}

/// A field: a commutative division ring with `0 != 1` (a law).
pub trait Field<Oa: Operator, Om: Operator>:
    DivisionRing<Oa, Om> + CommutativeRing<Oa, Om>
{
}

/// An ordered field: a field with a total order compatible with its
/// operations (the order laws are in `crate::laws`).
pub trait OrderedField<Oa: Operator, Om: Operator>: Field<Oa, Om> + PartialOrd {}

/// A finite field: a field with finitely many elements.
pub trait FiniteField<Oa: Operator, Om: Operator>: Field<Oa, Om> {
    /// The characteristic — the additive order of the multiplicative
    /// identity (a prime).
    fn characteristic() -> u64;

    /// The number of elements (a prime power).
    fn order() -> u64;
}

/// A field extension: a field that is a finite-dimensional vector space over
/// a base field.
pub trait FieldExtension<Oa: Operator, Om: Operator>:
    Field<Oa, Om> + VectorSpace<Oa, Om, Scalar = <Self as FieldExtension<Oa, Om>>::BaseField>
{
    /// The base field.
    type BaseField: Field<Oa, Om>;

    /// The degree of the extension `[K : F]`.
    fn degree() -> usize;

    /// The field trace `Tr(x)`.
    fn trace(&self) -> Self::BaseField;

    /// The field norm `N(x)`.
    fn norm(&self) -> Self::BaseField;
}

/// A tower of field extensions: `K = K₀ ⊃ K₁ ⊃ ... ⊃ K_h = F`.
pub trait FieldExtensionTower<Oa: Operator, Om: Operator>: FieldExtension<Oa, Om> {
    /// The height of the tower.
    fn tower_height() -> usize;

    /// The degree of the `i`-th step.
    fn extension_degree(i: usize) -> usize;
}
