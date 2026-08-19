//! Tuple impls: component-wise tower impls for `(T1, ..., Tn)`.
//!
//! One spec per trait covers every arity up to 16 (the serde ceiling; std
//! stops at 12 and there is no real-world demand beyond 16 — the lattice
//! specs cap at 12 because their `PartialOrd` supertrait does): the `()`
//! range generator mints the component generics, `where{@0..: ...}`
//! constrains every component from the first on (the range directly names
//! the generated components — no fresh-variable indirection), the
//! `impl{(A@..,)}` variadic template binds every component, and the body's
//! `@(...)..` repeat block emits one copy per component (name reference
//! `@A`, index cursor `@0`). The additive and multiplicative sides share
//! one block / one signature source. Tuples form rings when their components
//! do, but never fields (zero divisors).
//!
//! Note: std only derives `PartialEq`/`Debug`/etc. for tuples up to arity
//! 12, so 13..=16 tuples must be compared component-wise downstream; the
//! tower impls here are all component-wise and unaffected.
//!
use batch_impl::{batch_impl_only, batch_trait};

use crate::op::{Additive, Multiplicative};

use crate::tower::{
    AbelianGroup, CommutativeRing, Field, FiniteDimInnerSpace, FiniteDimVectorSpace, FreeModule,
    Group, InnerSpace, JoinSemilattice, Lattice, Loop, Magma, MeetSemilattice, Module, Monoid,
    NormedSpace, Quasigroup, Ring, Semigroup, Semiring, VectorSpace,
};

#[batch_impl_only(
    [
        Magma<Additive>,
        Magma<Multiplicative>
    ]^(<@trait<> >,)^1..=16 impl{(A@..,)}#combine{( @(@A::combine(&self.@0, &rhs.@0),).. )},
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    [
        Monoid<Additive>,
        Monoid<Multiplicative>
    ]^(<@trait<> >,)^1..=16 impl{(A@..,)} #identity{( @(@A::identity(),).. )},
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

// The multiplicative ladder stops at Monoid for tuples (componentwise
// inverses exist only additively for the numerics); the quasigroup ladder is
// additive-only too (`(R, ·)` is not a quasigroup: zero absorbs).

#[batch_impl_only(
    Group<Additive> (<@trait<> >,)^1..=16 impl{(A@..,)} #inverse{( @(@A::inverse(&self.@0),).. )},
)]
trait Group<Op: Operator>: Loop<Op> {
    fn inverse(&self) -> Self;
}

// ---- module level: componentwise scaling over the shared scalar ----
// `@1..` (open range) selects every component from the second one on; their
// `Scalar = @0::Scalar` value predicate pins them to the first component's
// scalar. Empty for arity 1 (no predicate, no error).

#[batch_impl_only(
    Module<Additive, Multiplicative> ()^1..=16 where{
        @0..: @trait<>,
        @1..: Module<Additive, Multiplicative, Scalar = @0::Scalar>,
    } impl{(A@..,)} #Scalar{A0::Scalar} #scale{( @(@A::scale(&s, v.@0),).. )},
)]
trait Module<Oa: Operator, Om: Operator>: AbelianGroup<Oa> {
    type Scalar;
    fn scale(s: &Self::Scalar, v: Self) -> Self;
}

// Marker levels need no directives and no duplicated trait signatures:
// one `batch_trait!` segment per trait.

batch_trait! {
    @tr_tup_to=(<@trait<> >,)^1..=;
    Semigroup: [Semigroup<Additive>, Semigroup<Multiplicative> ]^@tr_tup_to 16;
    Quasigroup: Quasigroup<Additive> @tr_tup_to 16;
    Loop: Loop<Additive> @tr_tup_to 16;
    AbelianGroup: AbelianGroup<Additive> @tr_tup_to 16;
    Semiring: Semiring<Additive, Multiplicative> @tr_tup_to 16;
    Ring: Ring<Additive, Multiplicative> @tr_tup_to 16;
    CommutativeRing: CommutativeRing<Additive, Multiplicative> @tr_tup_to 16;
    VectorSpace: VectorSpace<Additive, Multiplicative> @tr_tup_to 16 where{
        @1..: VectorSpace<Additive, Multiplicative,Scalar = @0::Scalar>,
        Self::Scalar: Field<Additive, Multiplicative>,
    };
    Lattice: @tr_tup_to 12;
    FiniteDimInnerSpace: FiniteDimInnerSpace<Additive, Multiplicative> (f64,)^1..=16;
}

// ---- analytic layer: componentwise lattices, norms, finite dimension ----

// Lattice specs cap at arity 12: `PartialOrd` (supertrait of the semilattice
// traits) is only implemented for tuples up to 12 by std. The algebraic
// tower above and the analytic layer below are std-tuple-trait-free, so they
// run to 16.

#[batch_impl_only(
    (<@trait>,)^1..=12 impl{(A@..,)} #meet{( @(@A::meet(&self.@0, &other.@0),).. )},
)]
trait MeetSemilattice: Sized + PartialOrd {
    fn meet(&self, other: &Self) -> Self;
}

#[batch_impl_only(
    (<@trait>,)^1..=12 impl{(A@..,)} #join{( @(@A::join(&self.@0, &other.@0),).. )},
)]
trait JoinSemilattice: Sized + PartialOrd {
    fn join(&self, other: &Self) -> Self;
}

// Normed/inner-product/finite-dimensional tuples: per-arity specs over the
// concrete `f64` components (the scalar reduction `Σ xᵢ²` is not expressible
// with the variadic repeat, and concrete types sidestep the associated-type
// equality chains) — the euclidean metric.

#[batch_impl_only(
    NormedSpace<Additive, Multiplicative> (f64,)^1..=16 impl{(A@..,)}
        #RealField{f64}
        #norm_squared{@(self.@0 * self.@0+)..0.}
    #scale_real{(@(<f64 as Module<Additive, Multiplicative>>::scale(&r, self.@0),)..)},
)]
trait NormedSpace<Oa: Operator, Om: Operator>: VectorSpace<Oa, Om> {
    type RealField;
    fn norm_squared(&self) -> Self::RealField;
    fn scale_real(&self, r: Self::RealField) -> Self;
}

#[batch_impl_only(
    InnerSpace<Additive, Multiplicative> (f64,)^1..=16 impl{(A@..,)}
        #inner_product{@(self.@0 * other.@0+)..0.},
)]
trait InnerSpace<Oa: Operator, Om: Operator>: NormedSpace<Oa, Om> {
    fn inner_product(&self, other: &Self) -> Self::RealField;
}

#[batch_impl_only(
    FiniteDimVectorSpace<Additive, Multiplicative> (f64,)^1..=16 impl{(A@..,)}
        #dimension{@(1+)..0}
        #canonical_basis_element{( @(if _i == @0 { 1.0 } else { 0.0 },).. )}
        #dot{@(self.@0 * other.@0+)..0.},
)]
trait FiniteDimVectorSpace<Oa: Operator, Om: Operator>: VectorSpace<Oa, Om> {
    fn dimension() -> usize;
    fn canonical_basis_element(_i: usize) -> Self;
    fn dot(&self, other: &Self) -> Self::Scalar;
}

// `(f64, ..., f64)` is the free module R^n: rank n, standard basis.

#[batch_impl_only(
    FreeModule<Additive, Multiplicative> (f64,)^1..=16 impl{(A@..,)}
        #rank{@(1+)..0}
        #basis_element{( @(if _i == @0 { 1.0 } else { 0.0 },).. )}
        #coordinate{match i { @( @0 => self.@0, ).. _ => unreachable!() }},
)]
trait FreeModule<Oa: Operator, Om: Operator>: Module<Oa, Om> {
    fn rank() -> usize;
    fn basis_element(_i: usize) -> Self;
    fn coordinate(&self, i: usize) -> Self::Scalar;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::{FiniteDimVectorSpace, NormedSpace};

    type A16 = (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32);
    type F16 = (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);

    #[test]
    fn arity_16_componentwise_add() {
        let a: A16 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let b: A16 = (16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1);
        // A16 is both Magma<Additive> and Magma<Multiplicative>: qualify.
        let s = <A16 as Magma<Additive>>::combine(&a, &b);
        // std stops tuple PartialEq at arity 12; compare components.
        assert_eq!(s.0, 17);
        assert_eq!(s.7, 17);
        assert_eq!(s.15, 17);
    }

    #[test]
    fn arity_16_finite_dim() {
        let v: F16 =
            (1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq!(F16::dimension(), 16);
        let e3 = F16::canonical_basis_element(2);
        assert_eq!(e3.0, 0.0);
        assert_eq!(e3.1, 0.0);
        assert_eq!(e3.2, 1.0);
        assert_eq!(e3.15, 0.0);
        // Σ i² for 1..=16 = 16·17·33/6 = 1496.
        assert_eq!(v.dot(&v), 1496.0);
        assert_eq!(v.norm_squared(), 1496.0);
    }

    #[test]
    fn free_module_f64_tuples() {
        use crate::tower::FreeModule;
        type F2 = (f64, f64);
        assert_eq!(<F2 as FreeModule<Additive, Multiplicative>>::rank(), 2);
        let e0 = <F2 as FreeModule<Additive, Multiplicative>>::basis_element(0);
        assert_eq!(e0.0, 1.0);
        assert_eq!(e0.1, 0.0);
        let e1 = <F2 as FreeModule<Additive, Multiplicative>>::basis_element(1);
        assert_eq!(e1.0, 0.0);
        assert_eq!(e1.1, 1.0);
        let v = (3.0f64, 4.0);
        assert_eq!(<F2 as FreeModule<Additive, Multiplicative>>::coordinate(&v, 0), 3.0);
        assert_eq!(<F2 as FreeModule<Additive, Multiplicative>>::coordinate(&v, 1), 4.0);
    }
}
