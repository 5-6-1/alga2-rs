# Changelog

All notable changes to this project are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-08

### Added

- `ModN<P>`: the integers modulo `P` — a finite field when `P` is prime,
  with extended-euclid inverse, characteristic/order, and euclidean division
  (Z/pZ is a principal ideal domain).
- `Quaternion<T>`: the non-commutative division ring (Hamiltonian product,
  normed algebra, Gram–Schmidt-compatible analytic layer).
- `[T; N]` arrays: the full component-wise tower for any `N` (no std tuple
  ceiling), plus the polynomial interface and free-module structure.
- Tuples 1–16: algebraic/module/analytic tiers (lattice and
  `Clone`/`PartialEq`-dependent tiers cap at 12, std's tuple ceiling).
- Extended structures: `StarSemiring` (Kleene star), `Band`, `LieAlgebra`,
  `EuclideanDomain`, `Power` (square-and-multiply).
- Field refinements: `OrderedField`, `FiniteField` (with `bool` = F₂),
  `ComplexField`, `FieldExtension`/`FieldExtensionTower`, `FreeModule`.
- Analytic layer: `Lattice`/`BoundedLattice`/`DistributiveLattice`/
  `ComplementedLattice`/`BooleanAlgebra`, `SubsetOf`/`SupersetOf`,
  `ClosedAdd`-family markers, `Real`, `BilinearForm` family,
  `TensorProduct`, the normed layer (`NormedSpace`/`InnerSpace`/
  `FiniteDimVectorSpace`/`FiniteDimInnerSpace`), `EuclideanSpace`/
  `AffineSpace`, and the `Matrix`/`Transformation` interfaces.
- `laws`: exported proptest law bundles — downstream users can check custom
  types against the laws of any hierarchy level (feature `proptest`).
- `alga2-derive`: `#[derive(Alga)]` brings a struct into the whole tower,
  component-wise, at a chosen level.
- Container matrices: `Vec`/`String` free monoids, `Box`/`Rc`/`Arc`
  smart-pointer delegation (`alloc`), `HashMap`/`HashSet`/`BTreeMap`/
  `BTreeSet` free monoids (`std`).

### Changed

- Default operator parameters throughout the tower: `Magma<Op = Additive>`,
  `Semiring<Oa = Additive, Om = Multiplicative>`, etc. — single-operator
  usage no longer needs to name the marker.
- The impl matrices moved to batch-impl's `batch_trait!`/`batch_impl_only`
  DSL with trait-name inheritance and in-constraint `@trait` references;
  only genuinely bulky algorithms (Hamiltonian product, extended euclid,
  component algebras) remain hand-written.

### Fixed

- `Complex` monoid identity: the imaginary part now uses the additive
  identity, so `Complex::identity()` is `1 + 0i`, not `1 + 1i`.
- `ModN` euclidean division intermediates run in `i128` and reduce via
  `rem_euclid` (usize wrapping overflow).
- `Complex<f32>`/`Quaternion<f32>` now match the `f64` coverage.

[0.1.0]: https://github.com/5-6-1/alga2/releases/tag/v0.1.0
