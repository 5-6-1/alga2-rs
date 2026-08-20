//! ModN impls: the integers modulo `P` — a finite field when `P` is prime.
//!
//! Everything rides on the residue arithmetic in `crate::modn`; the field
//! inverse is the extended-euclid modular inverse (exact when `P` is prime,
//! panics on a non-invertible residue otherwise).

use batch_impl::{batch_impl_only, batch_trait};

use crate::modn::ModN;
use crate::op::{Additive, Multiplicative};
use crate::tower::{
    AbelianGroup, CommutativeRing, DivisionRing, EuclideanDomain, Field, FiniteField, FreeModule,
    Group, IntegralDomain, Loop, Magma, Module, Monoid, PrincipalIdealDomain, Quasigroup, Ring,
    Semigroup, Semiring, UniqueFactorizationDomain, VectorSpace,
};

// ---- additive side: the residue ring Z/PZ ----

#[batch_impl_only(
    Magma<Additive> <const P: usize> ModN<P>
        #combine{ModN::new(self.value().wrapping_add(rhs.value()))},
    Magma<Multiplicative> <const P: usize> ModN<P>
        #combine{ModN::new(self.value().wrapping_mul(rhs.value()))},
)]
trait Magma<Op: Operator> {
    fn combine(&self, rhs: &Self) -> Self;
}

#[batch_impl_only(
    Monoid<Additive> <const P: usize> ModN<P>
        #identity{ModN::new(0)},
    Monoid<Multiplicative> <const P: usize> ModN<P>
        #identity{ModN::new(1)},
)]
trait Monoid<Op: Operator>: Semigroup<Op> {
    fn identity() -> Self;
}

#[batch_impl_only(
    Group<Additive> <const P: usize> ModN<P>
        #inverse{ModN::new(P.wrapping_sub(self.value()).wrapping_rem(P))},
)]
trait Group<Op: Operator>: Loop<Op> {
    fn inverse(&self) -> Self;
}

// The multiplicative inverse: extended euclid on `(self.value(), P)` — exact when
// `P` is prime (the residue is then coprime to `P`); for composite `P` a
// non-invertible residue yields a meaningless coefficient, so `Field` is
// documented as "for prime `P`".

#[batch_impl_only(
    DivisionRing<Additive, Multiplicative> <const P: usize> ModN<P>
        #inv{
            // Extended euclid on (self.value(), P); Bézout coefficients may
            // go negative, so they run in i128 and reduce via rem_euclid.
            let (mut old_r, mut r) = (self.value() as i128, P as i128);
            let (mut old_s, mut s) = (1i128, 0i128);
            while r != 0 {
                let q = old_r / r;
                (old_r, r) = (r, old_r - q * r);
                (old_s, s) = (s, old_s - q * s);
            }
            // old_r = gcd(self.value(), P) — 1 for prime P; old_s is the inverse.
            ModN::new(old_s.rem_euclid(P as i128) as usize)
        },
)]
trait DivisionRing<Oa: Operator, Om: Operator>: Ring<Oa, Om> {
    fn inv(&self) -> Self;
}

// ---- module level: a rank-1 free module over itself ----

#[batch_impl_only(
    Module<Additive, Multiplicative> <const P: usize> ModN<P>
        #Scalar{Self}
        #scale{ModN::new(s.value().wrapping_mul(v.value()))},
)]
trait Module<Oa: Operator, Om: Operator>: AbelianGroup<Oa> {
    type Scalar;
    fn scale(s: &Self::Scalar, v: Self) -> Self;
}

// Marker levels and simple methods: one `batch_trait!` segment per trait.

batch_trait! {
    Semigroup: Semigroup<Additive> <const P: usize> ModN<P>,
        Semigroup<Multiplicative> <const P: usize> ModN<P>;
    Quasigroup: Quasigroup<Additive> <const P: usize> ModN<P>;
    Loop: Loop<Additive> <const P: usize> ModN<P>;
    AbelianGroup: AbelianGroup<Additive> <const P: usize> ModN<P>;
    Semiring: Semiring<Additive, Multiplicative> <const P: usize> ModN<P>;
    Ring: Ring<Additive, Multiplicative> <const P: usize> ModN<P>;
    CommutativeRing: CommutativeRing<Additive, Multiplicative> <const P: usize> ModN<P>;
    Field: Field<Additive, Multiplicative> <const P: usize> ModN<P>;
    FiniteField: FiniteField<Additive, Multiplicative> <const P: usize> ModN<P>
        {fn characteristic() -> u64 { P as u64 } fn order() -> u64 { P as u64 }};
    IntegralDomain: IntegralDomain<Additive, Multiplicative> <const P: usize> ModN<P>;
    UniqueFactorizationDomain: UniqueFactorizationDomain<Additive, Multiplicative> <const P: usize> ModN<P>;
    PrincipalIdealDomain: PrincipalIdealDomain<Additive, Multiplicative> <const P: usize> ModN<P>;
    VectorSpace: VectorSpace<Additive, Multiplicative> <const P: usize> ModN<P>
        where{Self::Scalar: Field<Additive, Multiplicative>};
    FreeModule: FreeModule<Additive, Multiplicative> <const P: usize> ModN<P>
        {fn rank() -> usize { 1 } fn basis_element(_i: usize) -> Self { <Self as Monoid<Multiplicative>>::identity() } fn coordinate(&self, _i: usize) -> Self::Scalar { *self }};
}

// ---- EuclideanDomain: division by the modular inverse, remainder 0 (a
// ---- field has trivial euclidean division) ----

#[batch_impl_only(
    EuclideanDomain<Additive, Multiplicative> <const P: usize> ModN<P>
        #quot_rem{
            let inv = <Self as DivisionRing<Additive, Multiplicative>>::inv(divisor);
            (<Self as Magma<Multiplicative>>::combine(self, &inv), ModN::new(0))
        }
        #euclidean_norm{0},
)]
trait EuclideanDomain<Oa: Operator, Om: Operator>: CommutativeRing<Oa, Om> {
    fn quot_rem(&self, divisor: &Self) -> (Self, Self);
    fn euclidean_norm(&self) -> u128;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::{FiniteField, Group, Magma, PrincipalIdealDomain};

    fn add<const P: usize>(a: ModN<P>, b: ModN<P>) -> ModN<P> {
        <ModN<P> as Magma<Additive>>::combine(&a, &b)
    }

    fn mul<const P: usize>(a: ModN<P>, b: ModN<P>) -> ModN<P> {
        <ModN<P> as Magma<Multiplicative>>::combine(&a, &b)
    }

    #[test]
    fn residues_arithmetic() {
        // Z/7Z: 3 + 5 = 1, 3 · 5 = 1, −3 = 4.
        assert_eq!(add(ModN::<7>::new(3), ModN::new(5)), ModN::new(1));
        assert_eq!(mul(ModN::<7>::new(3), ModN::new(5)), ModN::new(1));
        let inv = <ModN<7> as Group<Additive>>::inverse(&ModN::new(3));
        assert_eq!(inv, ModN::new(4));
        // 5⁻¹ = 3 (mod 7): 5 · 3 = 15 ≡ 1.
        let five_inv = <ModN<7> as DivisionRing<Additive, Multiplicative>>::inv(&ModN::new(5));
        assert_eq!(five_inv, ModN::new(3));
        assert_eq!(mul(ModN::<7>::new(5), five_inv), ModN::new(1));
    }

    #[test]
    fn modn_is_a_finite_field() {
        // characteristic = order = P for the prime modulus.
        assert_eq!(<ModN<7> as FiniteField<Additive, Multiplicative>>::characteristic(), 7);
        assert_eq!(<ModN<7> as FiniteField<Additive, Multiplicative>>::order(), 7);
        // Field inverse: 2⁻¹ = 4 (mod 7).
        let inv = <ModN<7> as DivisionRing<Additive, Multiplicative>>::inv(&ModN::new(2));
        assert_eq!(inv, ModN::new(4));
    }

    #[test]
    fn modn_is_a_principal_ideal_domain() {
        // Z/pZ is a field, hence an integral domain / UFD / PID.
        fn assert_pid<const P: usize>()
        where
            ModN<P>: PrincipalIdealDomain<Additive, Multiplicative>,
        {
        }
        assert_pid::<7>();
        // Euclidean division on a field: a = (a·b⁻¹)·b + 0.
        let a = ModN::<7>::new(5);
        let b = ModN::new(3);
        let (q, r) = <ModN<7> as EuclideanDomain<Additive, Multiplicative>>::quot_rem(&a, &b);
        assert_eq!(mul(q, b), a);
        assert_eq!(r, ModN::new(0));
    }
}
