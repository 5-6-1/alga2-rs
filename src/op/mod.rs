//! The operator markers.
//!
//! Rust forbids implementing one trait twice for a type, but a type can be a
//! monoid under `+` AND under `*` — so every tower trait is parameterized
//! over an operator marker ([`Additive`] / [`Multiplicative`]), and the
//! ergonomic aliases (`AdditiveMonoid` etc.) arrive with the M1 naming audit.

/// Marker implemented by every operator tag.
///
/// Bounds the `Op` parameter of the tower traits (`Monoid<Op>` and friends),
/// so only operator markers can parameterize the hierarchy.
pub trait Operator {}

/// The additive operator (`+`).
pub struct Additive;

impl Operator for Additive {}

/// The multiplicative operator (`*`).
pub struct Multiplicative;

impl Operator for Multiplicative {}
