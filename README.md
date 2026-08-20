# alga2

A modern abstract-algebra hierarchy for Rust — the successor to
[alga](https://docs.rs/alga) (unmaintained since 2020), powered by
[batch-impl](https://docs.rs/batch-impl).

**~900 impls generated from ~80 batch-impl DSL blocks** — each method body
written once, the matrix supplying the quantity.

| Feature set | Impls | Types covered |
|---|---|---|
| bare core (`no_std`) | ~920 | 15 types (`@num` + `bool`/F₂) × full ladders + module/analytic layers, `[T; N]` arrays (any `N` — no std ceiling), tuples 1–16 (algebraic/module/analytic tiers; the lattice and `Clone`/`PartialEq`-dependent tiers cap at 12 — std's tuple-trait ceiling), `Option`, `Complex<T>`, `Quaternion<T>`, `ModN<P>` (Z/pZ, prime-modulus finite field) |
| `alloc` | +20 | `Vec<T>`, `String`, `Box<T>`, `Rc<T>`, `Arc<T>` (smart-pointer delegation) |
| `std` (default) | +8 | `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet` |

## Quick start

```toml
[dependencies]
alga2 = "0.1"
```

```rust
use alga2::op::{Additive, Multiplicative};
use alga2::tower::{DivisionRing, Group, Magma, Monoid};

// One type, two operators: `u8` is a monoid under both `+` and `*`.
assert_eq!(<u8 as Magma<Additive>>::combine(&3, &4), 7);
assert_eq!(<u8 as Magma<Multiplicative>>::combine(&3, &4), 12);
assert_eq!(<u8 as Monoid<Additive>>::identity(), 0);
assert_eq!(<u8 as Monoid<Multiplicative>>::identity(), 1);

// The additive ladder reaches groups: inverse and field inverse.
assert_eq!(<i32 as Group<Additive>>::inverse(&5), -5);
assert_eq!(<f64 as DivisionRing<Additive, Multiplicative>>::inv(&2.0), 0.5);
```

## The hierarchy

The tower traits are parameterized over operator markers
([`Additive`](crate::op::Additive) / [`Multiplicative`](crate::op::Multiplicative)),
so one type can inhabit each level twice without colliding:

- **additive ladder**: `Magma → Semigroup → Monoid → Group → AbelianGroup`
  (with the parallel `Quasigroup → Loop` leg — a group is an associative
  loop; `(R, ·)` is not a quasigroup, so the multiplicative side stops at
  Monoid)
- **multiplicative ladder**: `Magma → Semigroup → Monoid`
- **semiring ladder**: `Semiring → Ring → CommutativeRing → Field` (with the non-commutative `DivisionRing` — the quaternions are its classic inhabitant, provided in-crate)
- **module level**: `Module` (a ring acting on an abelian group) and
  `VectorSpace` (scalars form a field) — every numeric is a module over
  itself; tuples of same-scalar modules are modules, componentwise
- **extended structures**: `StarSemiring` (Kleene star), `Band` (idempotent semigroups), `EuclideanDomain` (euclidean division + gcd), `LieAlgebra` (bracket), `Power` (square-and-multiply)
- **field refinements**: `OrderedField`, `FiniteField` (with `bool` = F₂ as inhabitant), `ComplexField`, `FieldExtension`/`FieldExtensionTower` (C is a degree-2 extension of R, in-crate), `FreeModule` (the f64 tuples are Rⁿ)
- **analytic level**: `Lattice` (meet/join), `SubsetOf`/`SupersetOf`
  (numeric embeddings), `ClosedAdd`-family markers, `Real`, `BilinearForm`/
  `SymmetricBilinearForm`/`PositiveDefinite` (the multiplication form on the
  reals is the canonical example), `TensorProduct` (bilinearity laws), and
  the normed layer — `NormedSpace`/`InnerSpace`/`FiniteDimVectorSpace`/
  `FiniteDimInnerSpace` (Gram–Schmidt included), plus `EuclideanSpace`/
  `AffineSpace` and the `Matrix`/`Transformation` interfaces
  (`Isometry`/`DirectIsometry`/`OrthogonalTransformation`,
  `InversibleSquareMatrix`; alga-aligned, for downstream geometry types)

Integer operations are wrapping (mod 2^N): under `Additive`, `u8` is the
group Z/256Z, which plain `+` would break with an overflow panic in debug
builds.

## Law testing (`laws`)

Every level ships proptest properties and law bundles, so downstream users
check a custom type against the laws of the level it claims to implement:

```rust,ignore
// Requires the `proptest` feature; run inside your own test module.
use alga2::laws::ring_laws;
use proptest::prelude::*;

proptest! {
    fn my_type_obeys(a: MyType, b: MyType, c: MyType) {
        ring_laws(a, b, c)?;
    }
}
```

## Layout

- [`tower`](crate::tower) — the trait hierarchy (single source of truth)
- [`op`](crate::op) — the operator markers
- [`laws`](crate::laws) — proptest law testing (feature `proptest`)
- [`complex`](crate::complex) — in-crate `Complex<T>`, `Quaternion<T>` (zero-dependency)
- `impls` (private) — the batch-impl matrices

The core is `no_std`; container impls layer on via `alloc` and `std`
(default) features. Design notes: [docs/ROADMAP.md](docs/ROADMAP.md).
