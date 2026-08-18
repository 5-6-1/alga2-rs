//! Euclidean and affine spaces: points vs. vectors.
//!
//! A point lives in a `EuclideanSpace` with a coordinate vector space; an
//! affine space additionally distinguishes a translation vector space.

use crate::op::{Additive, Multiplicative};

use super::{Field, Group, Magma, Module, NormedSpace, VectorSpace};

/// An euclidean space: a set of points with a coordinate vector space and a
/// distance.
pub trait EuclideanSpace: Sized + Clone + PartialEq
where
    <Self::Coordinates as Module<Additive, Multiplicative>>::Scalar:
        Field<Additive, Multiplicative>,
{
    /// The vector space of coordinate differences.
    type Coordinates: NormedSpace<Additive, Multiplicative>
        + Module<Additive, Multiplicative>
        + VectorSpace<Additive, Multiplicative>
        + Clone;

    /// The origin point.
    fn origin() -> Self;

    /// The point at the given coordinates.
    fn from_coordinates(coords: Self::Coordinates) -> Self;

    /// The coordinates of `self` relative to the origin.
    fn coordinates(&self) -> Self::Coordinates;

    /// The euclidean distance to `other`: `‖other − self‖`.
    fn distance(
        &self, other: &Self,
    ) -> <Self::Coordinates as NormedSpace<Additive, Multiplicative>>::RealField {
        let diff = <Self::Coordinates as Magma<Additive>>::combine(
            &self.coordinates(),
            &<Self::Coordinates as Group<Additive>>::inverse(&other.coordinates()),
        );
        diff.norm()
    }
}

/// An affine space: points with a translation vector space.
pub trait AffineSpace: Sized + Clone + PartialEq
where
    <Self::Translation as Module<Additive, Multiplicative>>::Scalar:
        Field<Additive, Multiplicative>,
{
    /// The vector space of translations.
    type Translation: VectorSpace<Additive, Multiplicative>;

    /// The origin point.
    fn origin() -> Self;

    /// `origin` translated by `translation`.
    fn from_point_translation(origin: &Self, translation: &Self::Translation) -> Self;

    /// `self` translated by `translation`.
    fn translate_by(&self, t: &Self::Translation) -> Self;

    /// The translation taking `self` to `other`.
    fn translation(&self, other: &Self) -> Self::Translation;
}
