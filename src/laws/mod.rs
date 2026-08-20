//! Property-based law testing for the tower.
//!
//! Two layers:
//!
//! - **generic properties** — [`associativity`], [`monoid_identity`],
//!   [`group_inverse`], [`commutativity`], [`distributivity`]: one law each,
//!   parameterized over the operator(s);
//! - **law bundles** — [`additive_abelian_group_laws`],
//!   [`multiplicative_monoid_laws`], [`semiring_laws`], [`ring_laws`],
//!   [`commutative_ring_laws`], [`field_laws`]: the common operator
//!   combinations, one call per hierarchy level.
//!
//! Every function returns [`proptest::test_runner::TestCaseError`] and is
//! meant to be called from a `proptest!` block. This is the "strong"
//! differentiator of the crate: a downstream user checks a custom type
//! against the laws of the level it claims to implement, in a few lines:
//!
//! ```
//! use alga2::laws::{additive_abelian_group_laws, ring_laws};
//! use proptest::prelude::*;
//!
//! proptest! {
//!     fn i32_obeys(a: i32, b: i32, c: i32) {
//!         additive_abelian_group_laws(a, b, c)?;
//!         ring_laws(a, b, c)?;
//!     }
//! }
//! ```
//!
//! The laws are exact (`PartialEq`), which suits integers and other
//! precisely-computable types. Floats only approximate associativity and
//! distributivity, so the crate's own float tests check the exact laws
//! (identity, inverse, commutativity) plus a tolerance-based multiplicative
//! inverse instead of the bundles.

use core::fmt::Debug;

use proptest::prelude::*;

use crate::op::{Additive, Multiplicative, Operator};
use crate::tower::{
    AbelianGroup, Band, BilinearForm, CommutativeRing, ComplementedLattice, DistributiveLattice,
    DivisionRing, EuclideanDomain, Field, FieldExtension, FreeModule, Group, IntegralDomain,
    Lattice, LieAlgebra, LinearMap, Magma, Module, Monoid, OrderedField, PositiveDefinite,
    Quasigroup, Ring, Semiring, SymmetricBilinearForm, TensorProduct, VectorSpace,
};

/// Associativity: `(a·b)·c == a·(b·c)`.
pub fn associativity<Op: Operator, T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: Magma<Op> + Copy + PartialEq + Debug,
{
    let lhs = T::combine(&T::combine(&a, &b), &c);
    let rhs = T::combine(&a, &T::combine(&b, &c));
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

/// Identity: `e·a == a` and `a·e == a`.
pub fn monoid_identity<Op: Operator, T>(a: T) -> Result<(), TestCaseError>
where
    T: Monoid<Op> + Copy + PartialEq + Debug,
{
    let e = T::identity();
    prop_assert_eq!(T::combine(&e, &a), a);
    prop_assert_eq!(T::combine(&a, &e), a);
    Ok(())
}

/// Inverse: `a·a⁻¹ == e` and `a⁻¹·a == e`.
pub fn group_inverse<Op: Operator, T>(a: T) -> Result<(), TestCaseError>
where
    T: Group<Op> + Copy + PartialEq + Debug,
{
    let e = T::identity();
    let inv = T::inverse(&a);
    prop_assert_eq!(T::combine(&a, &inv), e);
    prop_assert_eq!(T::combine(&inv, &a), e);
    Ok(())
}

/// The latin-square property of a quasigroup: for every `a`, `b` there are
/// `x`, `y` with `a·x = b` and `y·a = b`. Constructed via the inverse for
/// group-backed quasigroups.
pub fn quasigroup_latin_square<Op: Operator, T>(a: T, b: T) -> Result<(), TestCaseError>
where
    T: Quasigroup<Op> + Group<Op> + Copy + PartialEq + Debug,
{
    let x = T::combine(&T::inverse(&a), &b);
    prop_assert_eq!(T::combine(&a, &x), b);
    let y = T::combine(&b, &T::inverse(&a));
    prop_assert_eq!(T::combine(&y, &a), b);
    Ok(())
}

/// Commutativity: `a·b == b·a`.
pub fn commutativity<Op: Operator, T>(a: T, b: T) -> Result<(), TestCaseError>
where
    T: AbelianGroup<Op> + Copy + PartialEq + Debug,
{
    prop_assert_eq!(T::combine(&a, &b), T::combine(&b, &a));
    Ok(())
}

/// Distributivity, both sides: `a·(b+c) == a·b + a·c` and
/// `(b+c)·a == b·a + c·a`.
pub fn distributivity<Oa: Operator, Om: Operator, T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: Semiring<Oa, Om> + Copy + PartialEq + Debug,
{
    let add = |x: &T, y: &T| <T as Magma<Oa>>::combine(x, y);
    let mul = |x: &T, y: &T| <T as Magma<Om>>::combine(x, y);
    // a·(b+c) == a·b + a·c
    prop_assert_eq!(mul(&a, &add(&b, &c)), add(&mul(&a, &b), &mul(&a, &c)));
    // (b+c)·a == b·a + c·a
    prop_assert_eq!(mul(&add(&b, &c), &a), add(&mul(&b, &a), &mul(&c, &a)));
    Ok(())
}

/// Every additive-ladder law through `AbelianGroup`.
pub fn additive_abelian_group_laws<T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: AbelianGroup<Additive> + Copy + PartialEq + Debug,
{
    associativity::<Additive, _>(a, b, c)?;
    monoid_identity::<Additive, _>(a)?;
    group_inverse::<Additive, _>(a)?;
    commutativity::<Additive, _>(a, b)?;
    Ok(())
}

/// Every multiplicative-ladder law through `Monoid` (the numerics stop there).
pub fn multiplicative_monoid_laws<T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: Monoid<Multiplicative> + Copy + PartialEq + Debug,
{
    associativity::<Multiplicative, _>(a, b, c)?;
    monoid_identity::<Multiplicative, _>(a)?;
    Ok(())
}

/// Distributivity with the default operators (`Additive` × `Multiplicative`).
pub fn semiring_laws<T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: Semiring<Additive, Multiplicative> + Copy + PartialEq + Debug,
{
    distributivity::<Additive, Multiplicative, _>(a, b, c)?;
    Ok(())
}

/// All ring laws: additive abelian group + multiplicative monoid + distributivity.
pub fn ring_laws<T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: Ring<Additive, Multiplicative> + Copy + PartialEq + Debug,
{
    additive_abelian_group_laws(a, b, c)?;
    multiplicative_monoid_laws(a, b, c)?;
    semiring_laws(a, b, c)?;
    Ok(())
}

/// Ring laws + multiplicative commutativity.
pub fn commutative_ring_laws<T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: CommutativeRing<Additive, Multiplicative> + Copy + PartialEq + Debug,
{
    ring_laws(a, b, c)?;
    prop_assert_eq!(
        <T as Magma<Multiplicative>>::combine(&a, &b),
        <T as Magma<Multiplicative>>::combine(&b, &a)
    );
    Ok(())
}

/// Field laws: commutative-ring laws + multiplicative inverse for nonzero
/// elements + `0 != 1`. Exact, so floats (approximate) should use a
/// tolerance-based check instead.
pub fn field_laws<T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: Field<Additive, Multiplicative> + Copy + PartialEq + Debug,
{
    commutative_ring_laws(a, b, c)?;
    let zero = <T as Monoid<Additive>>::identity();
    let one = <T as Monoid<Multiplicative>>::identity();
    prop_assert_ne!(zero, one);
    if a != zero {
        let inv = <T as DivisionRing<Additive, Multiplicative>>::inv(&a);
        prop_assert_eq!(<T as Magma<Multiplicative>>::combine(&a, &inv), one);
    }
    Ok(())
}

/// Module laws: scalar bilinearity + unit — `s·(u+v) == s·u + s·v`,
/// `(s+t)·v == s·v + t·v`, `(s·t)·v == s·(t·v)`, `1·v == v`.
pub fn module_laws<Oa: Operator, Om: Operator, T>(
    s: T::Scalar, t: T::Scalar, u: T, v: T,
) -> Result<(), TestCaseError>
where
    T: Module<Oa, Om> + Copy + PartialEq + Debug,
    T::Scalar: Copy + PartialEq + Debug,
{
    let add = |x: &T, y: &T| <T as Magma<Oa>>::combine(x, y);
    let scalar_add = |x: &T::Scalar, y: &T::Scalar| <T::Scalar as Magma<Oa>>::combine(x, y);
    let scalar_mul = |x: &T::Scalar, y: &T::Scalar| <T::Scalar as Magma<Om>>::combine(x, y);
    // s·(u+v) == s·u + s·v
    prop_assert_eq!(T::scale(&s, add(&u, &v)), add(&T::scale(&s, u), &T::scale(&s, v)));
    // (s+t)·v == s·v + t·v
    prop_assert_eq!(T::scale(&scalar_add(&s, &t), v), add(&T::scale(&s, v), &T::scale(&t, v)));
    // (s·t)·v == s·(t·v)
    prop_assert_eq!(T::scale(&scalar_mul(&s, &t), v), T::scale(&s, T::scale(&t, v)));
    // 1·v == v
    let one = <T::Scalar as Monoid<Om>>::identity();
    prop_assert_eq!(T::scale(&one, v), v);
    Ok(())
}

/// All module laws with the default operators (`Additive` × `Multiplicative`).
/// Exact, so floats (approximate bilinearity) should check the unit law
/// separately instead.
pub fn module_laws_default<T>(s: T::Scalar, t: T::Scalar, u: T, v: T) -> Result<(), TestCaseError>
where
    T: Module<Additive, Multiplicative> + Copy + PartialEq + Debug,
    T::Scalar: Copy + PartialEq + Debug,
{
    module_laws::<Additive, Multiplicative, _>(s, t, u, v)
}

/// Lattice absorption: `a ∧ (a ∨ b) == a` and `a ∨ (a ∧ b) == a`.
pub fn lattice_absorption<T>(a: T, b: T) -> Result<(), TestCaseError>
where
    T: Lattice + Copy + PartialEq + Debug,
{
    prop_assert_eq!(a.meet(&a.join(&b)), a);
    prop_assert_eq!(a.join(&a.meet(&b)), a);
    Ok(())
}

/// Band idempotence: `a·a == a`.
pub fn band_idempotent<Op: Operator, T>(a: T) -> Result<(), TestCaseError>
where
    T: Band<Op> + Copy + PartialEq + Debug,
{
    prop_assert_eq!(T::combine(&a, &a), a);
    Ok(())
}

/// Lie alternativity: `[a, a] == 0`.
pub fn lie_alternating<Op: Operator, T>(a: T) -> Result<(), TestCaseError>
where
    T: LieAlgebra<Op> + Magma<Additive> + Monoid<Additive> + Copy + PartialEq + Debug,
{
    let zero = <T as Monoid<Additive>>::identity();
    prop_assert_eq!(T::bracket(&a, &a), zero);
    Ok(())
}

/// The Jacobi identity: `[a,[b,c]] + [b,[c,a]] + [c,[a,b]] == 0`.
pub fn lie_jacobi<Op: Operator, T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: LieAlgebra<Op> + Magma<Additive> + Monoid<Additive> + Copy + PartialEq + Debug,
{
    let zero = <T as Monoid<Additive>>::identity();
    let abc = T::bracket(&a, &T::bracket(&b, &c));
    let bca = T::bracket(&b, &T::bracket(&c, &a));
    let cab = T::bracket(&c, &T::bracket(&a, &b));
    prop_assert_eq!(
        <T as Magma<Additive>>::combine(&<T as Magma<Additive>>::combine(&abc, &bca), &cab),
        zero
    );
    Ok(())
}

/// The euclidean division identity: `a == q·b + r` with `‖r‖ < ‖b‖`.
pub fn euclidean_division<Oa: Operator, Om: Operator, T>(a: T, b: T) -> Result<(), TestCaseError>
where
    T: EuclideanDomain<Oa, Om> + Copy + PartialEq + Debug + Monoid<Oa>,
{
    let zero = <T as Monoid<Oa>>::identity();
    if b == zero {
        return Ok(());
    }
    let (q, r) = a.quot_rem(&b);
    let back = <T as Magma<Om>>::combine(&q, &b);
    prop_assert_eq!(<T as Magma<Oa>>::combine(&back, &r), a);
    if r != zero {
        prop_assert!(r.euclidean_norm() < b.euclidean_norm());
    }
    Ok(())
}

/// Integral-domain law: no zero divisors — `a·b = 0` implies `a = 0` or
/// `b = 0` — and `0 != 1`.
pub fn integral_domain_laws<Oa: Operator, Om: Operator, T>(a: T, b: T) -> Result<(), TestCaseError>
where
    T: IntegralDomain<Oa, Om> + Copy + PartialEq + Debug + Monoid<Oa> + Monoid<Om>,
{
    let zero = <T as Monoid<Oa>>::identity();
    let one = <T as Monoid<Om>>::identity();
    prop_assert_ne!(zero, one);
    let ab = <T as Magma<Om>>::combine(&a, &b);
    if ab == zero {
        prop_assert!(a == zero || b == zero);
    }
    Ok(())
}

/// Distributivity of a lattice: `a ∧ (b ∨ c) == (a ∧ b) ∨ (a ∧ c)` and the
/// mirror `a ∨ (b ∧ c) == (a ∨ b) ∧ (a ∨ c)`.
pub fn lattice_distributivity<T>(a: T, b: T, c: T) -> Result<(), TestCaseError>
where
    T: DistributiveLattice + PartialEq + Debug,
{
    prop_assert_eq!(a.meet(&b.join(&c)), a.meet(&b).join(&a.meet(&c)));
    prop_assert_eq!(a.join(&b.meet(&c)), a.join(&b).meet(&a.join(&c)));
    Ok(())
}

/// Complement laws: `a ∨ ¬a == top` and `a ∧ ¬a == bottom`.
pub fn complemented_lattice_laws<T>(a: T) -> Result<(), TestCaseError>
where
    T: ComplementedLattice + PartialEq + Debug,
{
    prop_assert_eq!(a.join(&a.complement()), T::top());
    prop_assert_eq!(a.meet(&a.complement()), T::bottom());
    Ok(())
}

/// Linearity of a map: `f(u + v) == f(u) + f(v)`.
pub fn linear_map_additive<Oa: Operator, Om: Operator, M>(
    f: M, u: M::Domain, v: M::Domain,
) -> Result<(), TestCaseError>
where
    M: LinearMap<Oa, Om> + Clone,
    M::Domain: Magma<Oa> + PartialEq + Debug,
    M::Codomain: Magma<Oa> + PartialEq + Debug,
{
    let lhs = f.apply(&<M::Domain as Magma<Oa>>::combine(&u, &v));
    let rhs = <M::Codomain as Magma<Oa>>::combine(&f.apply(&u), &f.apply(&v));
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

/// Linearity of a map in the scalar: `f(s·u) == s·f(u)`.
pub fn linear_map_scalar<Oa: Operator, Om: Operator, M>(
    f: M, s: <M::Domain as Module<Oa, Om>>::Scalar, u: M::Domain,
) -> Result<(), TestCaseError>
where
    M: LinearMap<Oa, Om> + Clone,
    M::Domain: Module<Oa, Om> + Clone + PartialEq + Debug,
    M::Codomain: Module<Oa, Om> + PartialEq + Debug,
    <M::Domain as Module<Oa, Om>>::Scalar: Magma<Om> + Copy,
{
    let su = <M::Domain as Module<Oa, Om>>::scale(&s, u.clone());
    let lhs = f.apply(&su);
    let rhs = <M::Codomain as Module<Oa, Om>>::scale(&s, f.apply(&u));
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

/// Free-module basis law: the `j`-th coordinate of the `i`-th basis element
/// is the scalar `1` on the diagonal and `0` off it.
pub fn free_module_basis<Oa: Operator, Om: Operator, T>(
    i: usize, j: usize,
) -> Result<(), TestCaseError>
where
    T: FreeModule<Oa, Om>,
    T::Scalar: Monoid<Oa> + Monoid<Om> + PartialEq + Debug,
{
    let one = <T::Scalar as Monoid<Om>>::identity();
    let zero = <T::Scalar as Monoid<Oa>>::identity();
    let e_i = T::basis_element(i);
    if i == j {
        prop_assert_eq!(e_i.coordinate(j), one);
    } else {
        prop_assert_eq!(e_i.coordinate(j), zero);
    }
    Ok(())
}

/// Field-extension trace additivity: `Tr(x + y) == Tr(x) + Tr(y)`.
pub fn field_extension_trace_additive<Oa: Operator, Om: Operator, T>(
    x: T, y: T,
) -> Result<(), TestCaseError>
where
    T: FieldExtension<Oa, Om> + Copy + Magma<Oa>,
    T::BaseField: Magma<Oa> + PartialEq + Debug,
{
    let xy = <T as Magma<Oa>>::combine(&x, &y);
    let tr = <T::BaseField as Magma<Oa>>::combine(&x.trace(), &y.trace());
    prop_assert_eq!(xy.trace(), tr);
    Ok(())
}

/// Field-extension norm multiplicativity: `N(x·y) == N(x)·N(y)`.
pub fn field_extension_norm_multiplicative<Oa: Operator, Om: Operator, T>(
    x: T, y: T,
) -> Result<(), TestCaseError>
where
    T: FieldExtension<Oa, Om> + Copy + Magma<Om>,
    T::BaseField: Magma<Om> + PartialEq + Debug,
{
    let xy = <T as Magma<Om>>::combine(&x, &y);
    let n = <T::BaseField as Magma<Om>>::combine(&x.norm(), &y.norm());
    prop_assert_eq!(xy.norm(), n);
    Ok(())
}

/// Bilinearity on the left: `B(u1+u2, v) == B(u1, v) + B(u2, v)`.
pub fn bilinear_form_additive_left<F, S>(f: F, u1: S, u2: S, v: S) -> Result<(), TestCaseError>
where
    F: BilinearForm<Space = S>,
    <S as Module<Additive, Multiplicative>>::Scalar: Field<Additive, Multiplicative>,
    S: VectorSpace<Additive, Multiplicative> + Magma<Additive> + Clone + PartialEq + Debug,
    F::Scalar: Magma<Additive> + PartialEq + Debug,
{
    let lhs = f.apply(&<S as Magma<Additive>>::combine(&u1, &u2), &v);
    let rhs = <F::Scalar as Magma<Additive>>::combine(&f.apply(&u1, &v), &f.apply(&u2, &v));
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

/// Bilinearity on the right: `B(u, v1+v2) == B(u, v1) + B(u, v2)`.
pub fn bilinear_form_additive_right<F, S>(f: F, u: S, v1: S, v2: S) -> Result<(), TestCaseError>
where
    F: BilinearForm<Space = S>,
    <S as Module<Additive, Multiplicative>>::Scalar: Field<Additive, Multiplicative>,
    S: VectorSpace<Additive, Multiplicative> + Magma<Additive> + Clone + PartialEq + Debug,
    F::Scalar: Magma<Additive> + PartialEq + Debug,
{
    let lhs = f.apply(&u, &<S as Magma<Additive>>::combine(&v1, &v2));
    let rhs = <F::Scalar as Magma<Additive>>::combine(&f.apply(&u, &v1), &f.apply(&u, &v2));
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

/// Bilinearity in the scalar: `B(s·u, v) == s·B(u, v)`.
pub fn bilinear_form_scalar_left<F, S>(f: F, s: F::Scalar, u: S, v: S) -> Result<(), TestCaseError>
where
    F: BilinearForm<Space = S, Scalar = <S as Module<Additive, Multiplicative>>::Scalar>,
    <S as Module<Additive, Multiplicative>>::Scalar: Field<Additive, Multiplicative>,
    S: VectorSpace<Additive, Multiplicative> + Clone + PartialEq + Debug,
    F::Scalar: Magma<Multiplicative> + PartialEq + Debug + Copy,
{
    let su = <S as Module<Additive, Multiplicative>>::scale(&s, u.clone());
    let lhs = f.apply(&su, &v);
    let rhs = <F::Scalar as Magma<Multiplicative>>::combine(&s, &f.apply(&u, &v));
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

/// Symmetry: `B(u, v) == B(v, u)`.
pub fn bilinear_form_symmetric<F, S>(f: F, u: S, v: S) -> Result<(), TestCaseError>
where
    F: SymmetricBilinearForm<Space = S>,
    <S as Module<Additive, Multiplicative>>::Scalar: Field<Additive, Multiplicative>,
    S: VectorSpace<Additive, Multiplicative> + PartialEq + Debug,
    F::Scalar: PartialEq + Debug,
{
    prop_assert_eq!(f.apply(&u, &v), f.apply(&v, &u));
    Ok(())
}

/// Positive definiteness: `v != 0` implies `B(v, v) > 0`.
pub fn bilinear_form_positive_definite<F, S>(f: F, v: S) -> Result<(), TestCaseError>
where
    F: PositiveDefinite<Space = S>,
    <S as Module<Additive, Multiplicative>>::Scalar: Field<Additive, Multiplicative>,
    S: VectorSpace<Additive, Multiplicative> + Monoid<Additive> + Clone + PartialEq + Debug,
    F::Scalar: OrderedField<Additive, Multiplicative> + PartialOrd + Debug,
{
    let zero = <S as Monoid<Additive>>::identity();
    if v != zero {
        let b = f.apply(&v, &v);
        prop_assert!(b > <F::Scalar as Monoid<Additive>>::identity());
    }
    Ok(())
}

/// Tensor-product bilinearity on the left: `(u1+u2) ⊗ v == u1⊗v + u2⊗v`.
pub fn tensor_bilinear_left<Op: Operator, T>(
    u1: T::Left, u2: T::Left, v: T::Right,
) -> Result<(), TestCaseError>
where
    T: TensorProduct<Op> + Magma<Op> + PartialEq + Debug,
    T::Left: Magma<Op> + Clone,
    T::Right: Clone,
{
    let lhs = T::tensor_product(<T::Left as Magma<Op>>::combine(&u1, &u2), v.clone());
    let rhs =
        <T as Magma<Op>>::combine(&T::tensor_product(u1, v.clone()), &T::tensor_product(u2, v));
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

/// Tensor-product bilinearity on the right: `u ⊗ (v1+v2) == u⊗v1 + u⊗v2`.
pub fn tensor_bilinear_right<Op: Operator, T>(
    u: T::Left, v1: T::Right, v2: T::Right,
) -> Result<(), TestCaseError>
where
    T: TensorProduct<Op> + Magma<Op> + PartialEq + Debug,
    T::Left: Clone,
    T::Right: Magma<Op> + Clone,
{
    let lhs = T::tensor_product(u.clone(), <T::Right as Magma<Op>>::combine(&v1, &v2));
    let rhs =
        <T as Magma<Op>>::combine(&T::tensor_product(u.clone(), v1), &T::tensor_product(u, v2));
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complex::Complex;
    use crate::modn::ModN;

    proptest! {
        #[test]
        fn u8_laws(a: u8, b: u8, c: u8) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn u16_laws(a: u16, b: u16, c: u16) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn u32_laws(a: u32, b: u32, c: u32) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn u64_laws(a: u64, b: u64, c: u64) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn u128_laws(a: u128, b: u128, c: u128) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn usize_laws(a: usize, b: usize, c: usize) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn i8_laws(a: i8, b: i8, c: i8) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn i16_laws(a: i16, b: i16, c: i16) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn i32_laws(a: i32, b: i32, c: i32) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn i64_laws(a: i64, b: i64, c: i64) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn i128_laws(a: i128, b: i128, c: i128) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        #[test]
        fn isize_laws(a: isize, b: isize, c: isize) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            commutative_ring_laws(a, b, c)?;
        }

        // Floats: only the laws that are exact under IEEE 754 (identity,
        // inverse, commutativity) plus a tolerance-based multiplicative
        // inverse — associativity/distributivity are approximate.
        #[test]
        fn f32_laws(a: f32, b: f32, c: f32) {
            prop_assume!(a.is_finite() && b.is_finite() && c.is_finite());
            monoid_identity::<Additive, _>(a)?;
            group_inverse::<Additive, _>(a)?;
            commutativity::<Additive, _>(a, b)?;
            monoid_identity::<Multiplicative, _>(a)?;
            prop_assert_eq!(
                <f32 as Magma<Multiplicative>>::combine(&a, &b),
                <f32 as Magma<Multiplicative>>::combine(&b, &a)
            );
            // Subnormal magnitudes lose precision in `a * a⁻¹`; restrict the
            // inverse check to normal numbers.
            if a.abs() >= f32::MIN_POSITIVE {
                let inv = <f32 as DivisionRing<Additive, Multiplicative>>::inv(&a);
                prop_assert!((a * inv - 1.0).abs() < 1e-6);
            }
        }

        #[test]
        fn f64_laws(a: f64, b: f64, c: f64) {
            prop_assume!(a.is_finite() && b.is_finite() && c.is_finite());
            monoid_identity::<Additive, _>(a)?;
            group_inverse::<Additive, _>(a)?;
            commutativity::<Additive, _>(a, b)?;
            monoid_identity::<Multiplicative, _>(a)?;
            prop_assert_eq!(
                <f64 as Magma<Multiplicative>>::combine(&a, &b),
                <f64 as Magma<Multiplicative>>::combine(&b, &a)
            );
            // Subnormal magnitudes lose precision in `a * a⁻¹`; restrict the
            // inverse check to normal numbers.
            if a.abs() >= f64::MIN_POSITIVE {
                let inv = <f64 as DivisionRing<Additive, Multiplicative>>::inv(&a);
                prop_assert!((a * inv - 1.0).abs() < 1e-12);
            }
        }

        // Tuple matrices: component-wise impls inherit the laws from the
        // components (spot-checked here with the same bundles).
        #[test]
        fn tuple_laws(a: (u8, i16), b: (u8, i16), c: (u8, i16)) {
            additive_abelian_group_laws(a, b, c)?;
            multiplicative_monoid_laws(a, b, c)?;
            ring_laws(a, b, c)?;
        }

        #[test]
        fn triple_laws(a: (u8, i16, f32), b: (u8, i16, f32), c: (u8, i16, f32)) {
            // The mixed tuple still satisfies the additive ladder: f32's
            // approximate associativity is a real issue, so only check the
            // exact laws for tuples containing floats.
            monoid_identity::<Additive, _>(a)?;
            group_inverse::<Additive, _>(a)?;
            commutativity::<Additive, _>(a, b)?;
            monoid_identity::<Multiplicative, _>(a)?;
        }

        // Module level: exact on integers (scalar = vector for the numerics)
        // and on same-scalar tuples.
        #[test]
        fn u8_module_laws(s: u8, t: u8, u: u8, v: u8) {
            module_laws_default(s, t, u, v)?;
        }

        #[test]
        fn tuple_module_laws(s: u8, t: u8, u: (u8, u8), v: (u8, u8)) {
            module_laws_default(s, t, u, v)?;
        }

        #[test]
        fn tuple_vecspace_laws(s: f64, t: f64, u: (f64, f64), v: (f64, f64)) {
            // Bilinearity is approximate on floats; the unit law is exact.
            prop_assume!(
                s.is_finite() && t.is_finite() && u.0.is_finite() && u.1.is_finite()
                    && v.0.is_finite() && v.1.is_finite()
            );
            let one = 1.0f64;
            let scale = |w: (f64, f64)| <(f64, f64) as Module<Additive, Multiplicative>>::scale(&one, w);
            prop_assert_eq!(scale(u), u);
        }

        // `bool` is the exact two-element field F₂.
        #[test]
        fn bool_field_laws(a: bool, b: bool, c: bool) {
            field_laws(a, b, c)?;
            module_laws_default(a, b, a, b)?;
        }

        // Lattices: absorption on the numerics (exact) and tuples.
        #[test]
        fn u8_lattice_laws(a: u8, b: u8, c: u8) {
            lattice_absorption(a, b)?;
            lattice_distributivity(a, b, c)?;
        }

        #[test]
        fn tuple_lattice_laws(a: (u8, u8), b: (u8, u8)) {
            lattice_absorption(a, b)?;
        }

        #[test]
        fn bool_boolean_algebra(a: bool) {
            complemented_lattice_laws(a)?;
        }

        // Extended structures.
        #[test]
        fn bool_band_and_euclidean(a: bool, i: i32, j: i32) {
            band_idempotent::<Multiplicative, _>(a)?;
            euclidean_division(i, j)?;
            lie_alternating::<Additive, _>(i)?;
            lie_jacobi::<Additive, _>(i, j, 3)?;
        }

        // Free modules: the f64 tuple (f64, f64) = R².
        #[test]
        fn tuple_free_module_laws(i in 0usize..2, j in 0usize..2) {
            free_module_basis::<Additive, Multiplicative, (f64, f64)>(i, j)?;
        }

        // Bilinear forms: the multiplication form B(u, v) = u·v on f64.
        // The additive/scalar laws are approximate on floats, so only the
        // exact symmetry and positive-definiteness are property-tested
        // (positive-definiteness needs |u| large enough to square without
        // underflowing to zero).
        #[test]
        fn f64_bilinear_form_laws(f: f64, u: f64, v: f64) {
            prop_assume!(f.is_finite() && u.is_finite() && v.is_finite() && u.abs() > 1e-100);
            bilinear_form_symmetric(f, u, v)?;
            bilinear_form_positive_definite(f, u)?;
        }

        // Tensor products: a minimal pair-based implementation.
        #[test]
        fn tensor_product_laws(a: i32, b: i32, c: i32) {
            tensor_bilinear_left::<Additive, Tensor2>(a, b, c)?;
            tensor_bilinear_right::<Additive, Tensor2>(a, b, c)?;
        }

        // ModN: the prime-modulus finite field Z/97Z — exact laws (field,
        // integral domain, euclidean division) all hold.
        #[test]
        fn modn_field_laws(a: usize, b: usize, c: usize) {
            let a = ModN::<97>::new(a);
            let b = ModN::<97>::new(b);
            let c = ModN::<97>::new(c);
            field_laws(a, b, c)?;
            integral_domain_laws(a, b)?;
            euclidean_division(a, b)?;
        }

        // Linear maps: the scaling map v ↦ s·v on Z/97Z (exact).
        #[test]
        fn linear_map_laws(s: usize, u: usize, v: usize) {
            let s = ModN::<97>::new(s);
            let u = ModN::<97>::new(u);
            let v = ModN::<97>::new(v);
            let f = Scale(s);
            linear_map_additive::<Additive, Multiplicative, Scale>(f, u, v)?;
            linear_map_scalar::<Additive, Multiplicative, Scale>(f, s, u)?;
        }
    }

    // A scalar-scaling linear map: `v ↦ s·v` on the prime modulus field.
    #[derive(Clone, Copy)]
    struct Scale(ModN<97>);

    impl LinearMap<Additive, Multiplicative> for Scale {
        type Domain = ModN<97>;
        type Codomain = ModN<97>;

        fn apply(&self, v: &Self::Domain) -> Self::Codomain {
            <ModN<97> as Module<Additive, Multiplicative>>::scale(&self.0, *v)
        }
    }

    // Field extensions: C is a degree-2 extension of R. The trace/norm
    // identities are only exact on floats for exactly-representable
    // arithmetic, so they are spot-checked with small integers.
    #[test]
    fn complex_field_extension_exact() {
        let x = Complex::new(3.0f64, 4.0);
        let y = Complex::new(1.0, 2.0);
        // Tr((3+4i)+(1+2i)) = Tr(4+6i) = 8 == 6 + 2
        assert_eq!(Complex::new(4.0, 6.0).trace(), x.trace() + y.trace());
        // (3+4i)(1+2i) = -5+10i, N(-5+10i) = 125 == 25 · 5
        assert_eq!(Complex::new(-5.0, 10.0).norm(), x.norm() * y.norm());
    }

    // A minimal tensor product for law testing: the integer ring with
    // wrapping arithmetic, tensor = left·right (bilinear in the ring).
    #[derive(Clone, PartialEq, Debug)]
    struct Tensor2(i32, i32);

    impl Magma<Additive> for Tensor2 {
        fn combine(&self, rhs: &Self) -> Self {
            Tensor2(self.0.wrapping_add(rhs.0), self.1.wrapping_add(rhs.1))
        }
    }

    impl TensorProduct<Additive> for Tensor2 {
        type Left = i32;
        type Right = i32;

        fn tensor_product(left: i32, right: i32) -> Self {
            Tensor2(left.wrapping_mul(right), 0)
        }
    }
}
