//! The algebraic trait hierarchy (the "tower").
//!
//! Three ladders: the additive one here ([`Magma`] → [`Semigroup`] → [`Monoid`]
//! → [`Group`] → [`AbelianGroup`], with the [`Quasigroup`] → [`Loop`] leg),
//! the multiplicative one in [`semiring`] ([`Semiring`] → [`Ring`] →
//! [`CommutativeRing`] → [`Field`], plus the non-commutative
//! [`DivisionRing`]), and the module/vector-space level in [`module`] tying
//! both together. Every trait is parameterized over an [`Operator`] marker,
//! so one type can inhabit each level twice — once per operator — without
//! colliding.
//!
//! Beyond the core ladders: [`Lattice`]/[`BoundedLattice`],
//! [`StarSemiring`] (Kleene star), [`Band`] (idempotent semigroups),
//! [`EuclideanDomain`] (gcd), [`LieAlgebra`], [`Power`] (square-and-multiply),
//! the numeric embeddings [`SubsetOf`]/[`SupersetOf`], and the analytic
//! layer ([`NormedSpace`] → [`InnerSpace`] → [`FiniteDimVectorSpace`] →
//! [`FiniteDimInnerSpace`], [`EuclideanSpace`]/[`AffineSpace`]) plus the
//! [`Matrix`]/[`Transformation`] interfaces for downstream geometry types.
//!
//! Excluded by design (alga over-engineering, backlog only): none — the
//! original exclusions (Quasigroup/Loop/Band/Lattice) are now implemented.

pub mod band;
pub mod bilinear;
pub mod closed;
pub mod euclid;
pub mod euclidean;
pub mod id;
pub mod lattice;
pub mod lie;
pub mod matrix;
pub mod module;
pub mod norm;
pub mod polynomial;
pub mod pow;
pub mod real;
pub mod semiring;
pub mod star;
pub mod subset;
pub mod tensor;
pub mod transform;

pub use self::band::Band;
pub use self::bilinear::{BilinearForm, PositiveDefinite, SymmetricBilinearForm};
pub use self::closed::{ClosedAdd, ClosedDiv, ClosedMul, ClosedNeg, ClosedSub};
pub use self::euclid::{AffineSpace, EuclideanSpace};
pub use self::euclidean::EuclideanDomain;
pub use self::id::Id;
pub use self::lattice::{
    BooleanAlgebra, BoundedLattice, ComplementedLattice, DistributiveLattice, JoinSemilattice,
    Lattice, MeetSemilattice,
};
pub use self::lie::LieAlgebra;
pub use self::matrix::{InversibleSquareMatrix, Matrix, MatrixMut, SquareMatrix, SquareMatrixMut};
pub use self::module::{FreeModule, LinearMap, Module, VectorSpace};
pub use self::norm::{FiniteDimInnerSpace, FiniteDimVectorSpace, InnerSpace, NormedSpace};
pub use self::polynomial::Polynomial;
pub use self::pow::Power;
pub use self::real::{ComplexField, Real};
pub use self::semiring::{
    CommutativeRing, DivisionRing, Field, FieldExtension, FieldExtensionTower, FiniteField,
    IntegralDomain, OrderedField, PrincipalIdealDomain, Ring, Semiring, UniqueFactorizationDomain,
};
pub use self::star::StarSemiring;
pub use self::subset::{SubsetOf, SupersetOf};
pub use self::tensor::TensorProduct;
pub use self::transform::{
    AffineTransformation, DirectIsometry, Isometry, OrthogonalTransformation,
    ProjectiveTransformation, Rotation, Scaling, Similarity, Transformation, Translation,
};

use crate::op::Operator;

/// A magma: a set closed under a binary operation.
///
/// The operation itself lives in the impl: `combine` is `+` under
/// [`Additive`](crate::op::Additive) and `*` under
/// [`Multiplicative`](crate::op::Multiplicative).
pub trait Magma<Op: Operator> {
    /// The binary operation.
    fn combine(&self, rhs: &Self) -> Self;
}

/// A quasigroup: a [`Magma`] with the latin-square property — for every
/// `a`, `b` there is a unique `x` with `a·x = b` and a unique `y` with
/// `y·a = b` (a law in `crate::laws`). Every group is a quasigroup; note
/// that `(R, ·)` over the full numeric set is **not** one (zero absorbs),
/// which is why only the additive side of the matrix implements this.
pub trait Quasigroup<Op: Operator>: Magma<Op> {}

/// A loop: a [`Quasigroup`] with an identity element (a [`Monoid`]).
pub trait Loop<Op: Operator>: Quasigroup<Op> + Monoid<Op> {}

/// A semigroup: an associative [`Magma`].
pub trait Semigroup<Op: Operator>: Magma<Op> {}

/// A monoid: a [`Semigroup`] with an identity element.
pub trait Monoid<Op: Operator>: Semigroup<Op> {
    /// The identity element (`0` under `Additive`, `1` under `Multiplicative`).
    fn identity() -> Self;
}

/// A group: a [`Monoid`] with inverses (equivalently, an associative loop).
pub trait Group<Op: Operator>: Loop<Op> {
    /// The inverse of `self` (`-x` under `Additive`).
    fn inverse(&self) -> Self;
}

/// An abelian group: a [`Group`] whose operation commutes.
pub trait AbelianGroup<Op: Operator>: Group<Op> {}
