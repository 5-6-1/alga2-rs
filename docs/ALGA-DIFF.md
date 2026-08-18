# alga → alga2 API diff

Migration reference for [alga](https://docs.rs/alga) 0.9 users (the last
release, 2019–2020). alga2 keeps the *shape* of alga's design — operator
markers, abstract hierarchy, per-operator impls — while renaming the surface
and shrinking the trait count. Everything here is a compile-time mapping:
no behavior change beyond the renames below.

## Trait names

| alga (`alga::general`) | alga2 (`alga2::tower`) | Note |
|---|---|---|
| `AbstractMagma<O>` | `Magma<Op>` | `Abstract` prefix dropped |
| `AbstractSemigroup<O>` | `Semigroup<Op>` | |
| `AbstractMonoid<O>` | `Monoid<Op>` | |
| `AbstractGroup<O>` | `Group<Op>` | |
| `AbstractAbelianGroup<O>` | `AbelianGroup<Op>` | |
| `AbstractQuasigroup<O>` / `AbstractLoop<O>` | — | out of v0.1 scope by design (backlog) |
| `AbstractRing<Oa, Om>` | `Ring<Oa, Om>` | |
| `AbstractField<Oa, Om>` | `Field<Oa, Om>` | |
| `Operator`, `Additive`, `Multiplicative` | `alga2::op::{Operator, Additive, Multiplicative}` | unchanged |

## Method names

alga 0.9 splits the identity/inverse operations into standalone traits
(`Identity`, `Inverse`, `TwoSidedInverse`); alga2 folds them back into the
hierarchy as plain methods — one fewer trait set to import and bound.

| alga | alga2 | Where |
|---|---|---|
| `<T as AbstractMagma<O>>::operate(&self, right)` | `Magma::combine(&self, rhs)` | hierarchy method |
| `Identity::identity()` | `Monoid::identity()` | hierarchy method |
| `TwoSidedInverse::two_sided_inverse()` / `Inverse::inverse()` | `Group::inverse(&self)` | hierarchy method |
| `Field::mul_inv(&self)` (if present) | `Field::inv(&self)` | hierarchy method |

## Semantics and structure

- **Wrapping integers**: alga2's integer impls are wrapping (mod 2^N) so the
  additive ladder is exact in debug builds (`u8` is Z/256Z); alga used plain
  `+`/`*`, which panicked on overflow in debug.
- **`Ring` requires an abelian additive group** at the trait level
  (`Ring<Oa, Om>: Semiring<Oa, Om> + AbelianGroup<Oa>`), matching the
  mathematical definition; alga's `AbstractRing` only required a group.
- **`laws` module**: alga's quickcheck properties become exported proptest
  law bundles (`alga2::laws`) — downstream users can check custom types.
- **Impl generation**: alga's hand-written impls are batch-impl matrices
  (see the `impls` module docs) — the generated surface is equivalent.

## Mechanical migration sketch

```rust
// alga
use alga::general::{AbstractGroup, Additive};
let inv = <i32 as AbstractGroup<Additive>>::two_sided_inverse(&5);

// alga2
use alga2::op::Additive;
use alga2::tower::Group;
let inv = <i32 as Group<Additive>>::inverse(&5);
```

Renames are mechanical (`AbstractX` → `X`, `operate` → `combine`,
`two_sided_inverse` → `inverse`); a compat shim crate can provide the old
names on top of alga2 once the core settles.
