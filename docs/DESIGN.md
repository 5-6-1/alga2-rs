# alga2 design

This document explains *how* alga2 is built: the algebraic model, the
impl-generation strategy, and the DSL idioms the matrices rely on. It is
the "why" behind `src/tower` and `src/impls`; the roadmap covers *where the
project is going* (`ROADMAP.md`) and the alga migration mapping is in
`ALGA-DIFF.md`.

## 1. The algebraic model

### 1.1 Operators, not traits

A single Rust type can be a monoid under `+` *and* under `*` (`u8` is both
Z/256Z under addition and a monoid under multiplication). Rust forbids
implementing one trait twice for a type, so every tower trait is
parameterized over an **operator marker**:

```rust
pub trait Magma<Op: Operator = Additive> {
    fn combine(&self, rhs: &Self) -> Self;
}
```

`Additive` and `Multiplicative` are empty marker types (`src/op`). The
hierarchy is then inhabited twice per type without collision:
`Magma<Additive>` is `+`, `Magma<Multiplicative>` is `*`.

Default parameters (`= Additive`, and `= Multiplicative` for the second
parameter of `Semiring`-family traits) make single-operator usage terse:
`fn f<T: Monoid>(x: T)` already means the additive monoid.

### 1.2 The ladders

```
additive:      Magma → Semigroup → Monoid → Group → AbelianGroup
               (plus the Quasigroup → Loop leg — a group is an
               associative loop; (R, ·) is not a quasigroup: zero absorbs)
multiplicative: Magma → Semigroup → Monoid            (numerics stop here)
semiring:      Semiring → Ring → CommutativeRing → Field
               (+ non-commutative DivisionRing — the quaternions)
module level:  Module → VectorSpace → FreeModule       (ring acting on group)
field refinements: OrderedField, FiniteField, ComplexField,
               FieldExtension / FieldExtensionTower
extended:      StarSemiring, Band, EuclideanDomain, LieAlgebra, Power
analytic:      Lattice family, SubsetOf/SupersetOf, ClosedAdd family, Real,
               BilinearForm family, TensorProduct, NormedSpace → InnerSpace
               → FiniteDimVectorSpace → FiniteDimInnerSpace,
               EuclideanSpace/AffineSpace, Matrix/Transformation interfaces
```

Every trait lives in `src/tower/<name>.rs`; `src/tower/mod.rs` re-exports
the public surface. The tower is the **single source of truth** — impls
only implement, never redefine.

### 1.3 Laws

Each level carries a proptest law bundle in `src/laws` (`associativity`,
`monoid_identity`, `group_inverse`, `commutativity`, `distributivity`, the
per-level bundles, `module_laws`, lattice/band/lie/euclidean/star/field
laws, bilinear-form and tensor-product laws). The law layer is the crate's
differentiator: downstream users run the same bundles against *their* types
via the `proptest` feature.

## 2. The impl-generation strategy

### 2.1 Zero duplication is the contract

Every method body is written **once**. The matrix supplies the quantity —
types, operators, arities — and batch-impl the expansion. This is not a
style preference; it is the project's hard rule (see `CONTRIBUTING.md`).

### 2.2 Three entry points, in decreasing preference

From `src/impls/mod.rs`:

1. **Hand-written `impl`s** — only genuinely bulky algorithm bodies where
   the DSL would obscure the math: the Hamiltonian product, extended euclid
   (ModN inverse), component algebras. These are few and clearly marked.
2. **`batch_trait!`** — everything simple: marker levels, one-line method
   bodies, associated types, smart-pointer shape templates. The bulk.
3. **`batch_impl_only`** — only where the directive system earns its keep:
   `tuples.rs`'s variadic-repeat component matrix (per-arity generation no
   other form expresses) and `SubsetOf`'s shared `T`-parameterized bodies
   (13 entries, three bodies — zero duplication).

### 2.3 Reuse degree decides the boundary

The decision rule for where a body goes:

- **High reuse + similar structure** → `batch_trait!` (up to ~60-line
  bodies: the complex division-ring inverse and the normed layer).
- **Low reuse** → hand-written `impl`s (the ~40-line boundary): the
  Hamiltonian product (55 lines, single impl) is hand-written precisely
  because it appears once.

## 3. DSL idioms (batch-impl showcase)

These are the patterns the matrices use. They are the *result* of alga2's
usage pushing batch-impl forward; new features were contributed upstream as
the crate grew.

### 3.1 Constants and trait references

User constants shrink repeated shapes:

```
@am=Additive, Multiplicative;
@pri=[@num, bool];
@pmod=<const P: usize> ModN<P>;
```

`@trait` references the *current spec's trait name*; the target follows
(space-separated — no dot):

```
Ring: <T: @trait> Complex<T>;        // T: Ring (default params)
Field: [@f*, bool];                   // Field<Additive, Multiplicative>
```

Inside a constraint, `@trait` means "this spec's trait with its default
parameters" — the trait name is written once, at the segment head.

### 3.2 Trait-name inheritance

The segment head names the trait once; every following spec inherits it.
No `@trait` prefix needed:

```
Semiring: @pmod;        // impl Semiring<Additive, Multiplicative> for ModN<P>
Quasigroup: @pri;       // impl Quasigroup<Additive> for the numerics
DivisionRing: @pmod;    // impl DivisionRing<Additive, Multiplicative> ...
```

Traits *without* default parameters (`Module`, `VectorSpace`,
`FreeModule`, `InnerSpace`, `Power`, `LieAlgebra`, …) still write their
arguments explicitly: `Module: @trait<@am> <T: Field> Quaternion<T>`.

### 3.3 Shape templates and lists

`impl{...}` rewrites a body's self/arguments per wrapper — `impl{Box<T>}`
turns `Box::new` into `Rc::new`/`Arc::new`:

```
@ptr=[Box,Rc,Arc];
@impl=<T:@trait<>> @ptr T;
Magma: <T: Clone> Vec<T>{ ... },
    <Op: Operator> @trait<Op> @impl impl{Box<_>}{ ... };
```

Lists distribute specs: `@trait[<Additive>,<Multiplicative>]` generates one
impl per operator; multi-spec segments are comma-separated (the `[...]`
wrapper is optional). `[A,B]` type lists spread bodies over type families.

### 3.4 The X<> sync rule

Inside a spec body, `Trait<Op>` refers to the *same* `Op` that the spec was
generated for, whatever it is:

```
@with_mul=@trait<Multiplicative> <T: Ring> Complex<T>;
Magma: @with_mul impl{@trait<>}{
    fn combine(...) { ... <T as Magma<>>::combine(self.re(), rhs.re()) ... }
}
```

`<T as Magma<>>` (empty brackets) here means `Magma<Multiplicative>` — the
spec's operator — while `<T as Magma>` (bare) would mean the default
`Magma<Additive>`. The two forms are **not interchangeable**; the combine
body in `src/impls/complex.rs` deliberately mixes them (outer combine is
the spec operator, inner combines are the defaults). A different-named
trait with empty brackets switches to the spec's full argument list.

### 3.5 Directives (`batch_impl_only`)

`batch_impl_only` keeps the trait signature local and drives generation
with `#combine`-style directives and position markers. The tuple matrix is
the showcase: one block per level, arity 1–16 via the variadic-repeat
range `(<@trait<>>,).1..=16` and `@0..`/`@1..` position references:

```
@trait[<Additive>,<Multiplicative>]
(<@trait<>>,).1..=16 impl{(A@..,)} #combine{( @(@A::combine(&self.@0, &rhs.@0),).. )},
```

## 4. Feature layering and no_std

| Feature | Gate | Contents |
|---|---|---|
| bare (no features) | — | primitives, tuples 1–16, Option, Complex, Quaternion, ModN, arrays, analytics |
| `alloc` | — | Vec/String/Box/Rc/Arc (smart-pointer delegation) |
| `std` (default) | `std = ["alloc"]` | HashMap/HashSet/BTreeMap/BTreeSet |
| `proptest` | `proptest = ["dep:proptest", "std"]` | exports `src/laws` to downstream users |

The bare core has **zero dependencies** (batch-impl is build-time only;
proptest never leaks into downstream builds). `#![forbid(unsafe_code)]` is
crate-wide.

## 5. Known trade-offs

- **Exact law bundles are for exact arithmetic.** Floats only approximate
  associativity/distributivity, so the float tests check the exact laws
  (identity, inverse, commutativity) plus a tolerance-based inverse instead
  of the full bundles. Law bundles document this.
- **Wrapping integers by design.** Under `Additive`, `u8` is Z/256Z;
  plain `+` would panic in debug on overflow. `ModN<P>` gives the explicit
  quotient when a specific modulus is wanted.
- **Tuple ceiling at 12** for `Clone`/`PartialOrd`-dependent tiers is std's
  tuple-trait ceiling, not ours; purely algebraic tiers run to 16.
- **The `Ring` supertrait is the strict one**: `Ring<Oa, Om>:
  Semiring<Oa, Om> + AbelianGroup<Oa>` matches the mathematical definition,
  where alga's `AbstractRing` only required a group.

## 6. Reading order

1. `README.md` — the surface.
2. `src/tower/mod.rs` — the hierarchy in one screen.
3. `src/impls/complex.rs` — constants, `@trait` references, X<> sync, and
   the hand-written/batch_trait! boundary in one file.
4. `src/impls/tuples.rs` — the directive showcase.
5. `src/laws/mod.rs` — how every level is checked.
