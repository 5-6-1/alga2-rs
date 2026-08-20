//! Level coverage: what each `#[alga(level = ...)]` target generates and
//! enforces. A struct at level L must be usable as every trait up to L.

use alga2_derive::Alga;

#[derive(Alga, PartialEq, Debug)]
#[alga(level = "Monoid")]
struct AddMon(u8);

#[derive(Alga, PartialEq, Debug)]
#[alga(level = "Group")]
struct G(i32);

#[derive(Alga, PartialEq, Debug)]
#[alga(level = "Ring")]
struct R(i32);

#[derive(Alga, PartialEq, Debug)]
#[alga(level = "Field")]
struct F(f64);

#[test]
fn monoid_level() {
    fn assert_monoid<T: alga2::tower::Monoid<alga2::op::Additive>>() {}
    assert_monoid::<AddMon>();
    fn assert_no_group<T: alga2::tower::Group<alga2::op::Additive>>() {}
    // assert_no_group::<AddMon>(); // uncomment → compile error (expected)
    let _ = assert_no_group::<G>;
}

#[test]
fn group_level() {
    let a = G(5);
    let inv = <G as alga2::tower::Group<alga2::op::Additive>>::inverse(&a);
    assert_eq!(inv, G(-5));
    // Multiplicative ladder still stops at Monoid (no field inverse on i32).
    fn assert_mul_monoid<T: alga2::tower::Monoid<alga2::op::Multiplicative>>() {}
    assert_mul_monoid::<G>();
    // Ring is not implemented at Group level.
    fn assert_no_ring<T: alga2::tower::Ring<alga2::op::Additive, alga2::op::Multiplicative>>() {}
    let _ = assert_no_ring::<R>;
}

#[test]
fn ring_level() {
    fn assert_ring<T: alga2::tower::Ring<alga2::op::Additive, alga2::op::Multiplicative>>() {}
    assert_ring::<R>();
    fn assert_comm<
        T: alga2::tower::CommutativeRing<alga2::op::Additive, alga2::op::Multiplicative>,
    >() {
    }
    assert_comm::<R>();
    fn assert_module<T: alga2::tower::Module<alga2::op::Additive, alga2::op::Multiplicative>>() {}
    assert_module::<R>();
    fn assert_no_field<T: alga2::tower::Field<alga2::op::Additive, alga2::op::Multiplicative>>() {}
    let _ = assert_no_field::<F>;
}

#[test]
fn field_level() {
    fn assert_field<T: alga2::tower::Field<alga2::op::Additive, alga2::op::Multiplicative>>() {}
    assert_field::<F>();
    let a = F(2.0);
    let inv =
        <F as alga2::tower::DivisionRing<alga2::op::Additive, alga2::op::Multiplicative>>::inv(&a);
    assert_eq!(inv, F(0.5));
    // Module scale on the vector.
    let sc =
        <F as alga2::tower::Module<alga2::op::Additive, alga2::op::Multiplicative>>::scale(&3.0, a);
    assert_eq!(sc, F(6.0));
}

#[test]
fn tuple_struct_fields() {
    #[derive(Alga, PartialEq, Debug)]
    #[alga(level = "Field")]
    struct Pair(f64, f64);
    let a = Pair(1.0, 2.0);
    let b = Pair(3.0, 4.0);
    let s = <Pair as alga2::tower::Magma<alga2::op::Additive>>::combine(&a, &b);
    assert_eq!(s, Pair(4.0, 6.0));
    let inv = <Pair as alga2::tower::DivisionRing<
        alga2::op::Additive,
        alga2::op::Multiplicative,
    >>::inv(&a);
    assert_eq!(inv, Pair(1.0, 0.5));
}

#[test]
fn default_level_is_ring() {
    #[derive(Alga)]
    struct DefaultLevel(i32);
    fn assert_ring<T: alga2::tower::Ring<alga2::op::Additive, alga2::op::Multiplicative>>() {}
    assert_ring::<DefaultLevel>();
}
