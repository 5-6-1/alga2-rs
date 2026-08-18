//! Transformations of points/vectors (alga-aligned interface).
//!
//! The transformation hierarchy — plain, projective, affine, similarity —
//! describes how a transformation acts on an [`EuclideanSpace`]: points and
//! vectors are transformed separately, and the richer classes decompose
//! into translation / rotation / scaling. Like the matrix traits, these are
//! an interface for downstream geometry types.

use crate::tower::EuclideanSpace;

/// A transformation of an euclidean space: acts on both points and vectors.
pub trait Transformation<E: EuclideanSpace>: Sized + Clone {
    /// Transforms a point.
    fn transform_point(&self, pt: &E) -> E;

    /// Transforms a vector (the coordinate part).
    fn transform_vector(&self, v: &E::Coordinates) -> E::Coordinates;
}

/// A projective transformation: invertible.
pub trait ProjectiveTransformation<E: EuclideanSpace>: Transformation<E> {
    /// The inverse transformation.
    fn inverse_transform(&self) -> Self;

    /// Transforms a point by the inverse.
    fn inverse_transform_point(&self, pt: &E) -> E {
        self.inverse_transform().transform_point(pt)
    }

    /// Transforms a vector by the inverse.
    fn inverse_transform_vector(&self, v: &E::Coordinates) -> E::Coordinates {
        self.inverse_transform().transform_vector(v)
    }
}

/// An affine transformation: decomposes into translation, rotation, scaling.
pub trait AffineTransformation<E: EuclideanSpace>: ProjectiveTransformation<E> {
    /// The rotation part type.
    type Rotation: Rotation<E>;

    /// The non-uniform scaling part type.
    type NonUniformScaling: AffineTransformation<E>;

    /// The translation part type.
    type Translation: Translation<E>;

    /// Decomposes into `(translation, rotation, non-uniform scaling)`.
    fn decompose(&self) -> (Self::Translation, Self::Rotation, Self::NonUniformScaling);

    /// Appends a translation.
    fn append_translation(&self, t: &Self::Translation) -> Self;

    /// Prepends a translation.
    fn prepend_translation(&self, t: &Self::Translation) -> Self;

    /// Appends a rotation.
    fn append_rotation(&self, r: &Self::Rotation) -> Self;

    /// Prepends a rotation.
    fn prepend_rotation(&self, r: &Self::Rotation) -> Self;

    /// Appends a non-uniform scaling.
    fn append_scaling(&self, s: &Self::NonUniformScaling) -> Self;

    /// Prepends a non-uniform scaling.
    fn prepend_scaling(&self, s: &Self::NonUniformScaling) -> Self;
}

/// A similarity transformation: uniform scaling + rotation + translation.
pub trait Similarity<E: EuclideanSpace>: AffineTransformation<E> {
    /// The uniform scaling part type.
    type Scaling: Scaling<E>;

    /// The translation part.
    fn translation(&self) -> Self::Translation;

    /// The rotation part.
    fn rotation(&self) -> Self::Rotation;

    /// The uniform scaling part.
    fn scaling(&self) -> Self::Scaling;

    /// Translates a point.
    fn translate_point(&self, pt: &E) -> E {
        <Self::Translation as Translation<E>>::transform_point(&self.translation(), pt)
    }

    /// Rotates a point.
    fn rotate_point(&self, pt: &E) -> E {
        <Self::Rotation as Transformation<E>>::transform_point(&self.rotation(), pt)
    }

    /// Uniformly scales a point.
    fn scale_point(&self, pt: &E) -> E {
        <Self::Scaling as Transformation<E>>::transform_point(&self.scaling(), pt)
    }
}

/// A rotation: preserves norms and orientation.
pub trait Rotation<E: EuclideanSpace>: Transformation<E> {}

/// An isometry: a similarity with unit scaling (preserves distances).
pub trait Isometry<E: EuclideanSpace>: Similarity<E> {}

/// A direct isometry: an isometry preserving orientation.
pub trait DirectIsometry<E: EuclideanSpace>: Isometry<E> {}

/// An orthogonal transformation: preserves the inner product — a linear
/// isometry fixing the origin.
pub trait OrthogonalTransformation<E: EuclideanSpace>: Transformation<E> {}

/// A (possibly non-uniform) scaling.
pub trait Scaling<E: EuclideanSpace>: Transformation<E> {}

/// A translation: moves points by a vector, leaves vectors untouched.
pub trait Translation<E: EuclideanSpace>: Transformation<E> {
    /// The translation vector.
    fn translation_vector(&self) -> E::Coordinates;

    /// Applies to a point.
    fn transform_point(&self, pt: &E) -> E {
        E::from_coordinates(<E::Coordinates as crate::tower::Magma<crate::op::Additive>>::combine(
            &pt.coordinates(),
            &self.translation_vector(),
        ))
    }

    /// Applies to a vector (the identity).
    fn transform_vector(&self, v: &E::Coordinates) -> E::Coordinates {
        <E::Coordinates as Clone>::clone(v)
    }
}
