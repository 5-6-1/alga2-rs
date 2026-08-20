//! `#[derive(Alga)]` end-to-end: a struct gets the whole tower, component-wise.

use alga2::op::{Additive, Multiplicative};
use alga2::tower::{DivisionRing, Group, Magma, Module, Monoid, Ring};
use alga2_derive::Alga;

#[derive(Alga)]
#[alga(level = "Field")]
struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Alga)]
#[alga(level = "Field")]
struct Pair(f64, f64);

#[test]
fn named_struct_derives_full_field() {
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    // Two Magma impls exist (Additive and Multiplicative): qualify.
    let s = <Vec2 as Magma<Additive>>::combine(&a, &b);
    assert_eq!(s.x, 4.0);
    assert_eq!(<Vec2 as Monoid<Additive>>::identity().x, 0.0);
    assert_eq!(<Vec2 as Group<Additive>>::inverse(&a).x, -1.0);
    // Field inverse is component-wise: inv(2) = 0.5.
    let inv = <Vec2 as DivisionRing<Additive, Multiplicative>>::inv(&Vec2 { x: 2.0, y: 4.0 });
    assert_eq!(inv.x, 0.5);
    // Module scale.
    let sc = <Vec2 as Module<Additive, Multiplicative>>::scale(&2.0, a);
    assert_eq!(sc.x, 2.0);
    assert_eq!(sc.y, 4.0);
}

#[test]
fn tuple_struct_derives() {
    let p = Pair(1.0, 2.0);
    let q = Pair(3.0, 4.0);
    let s = <Pair as Magma<Additive>>::combine(&p, &q);
    assert_eq!(s.0, 4.0);
    assert_eq!(<Pair as Monoid<Additive>>::identity().1, 0.0);
}

// `i32` fields only reach `Ring` (they are not a field): the derived `Field`
// impl is gated by `where i32: Field`, so it simply does not apply — and the
// tower below still works.
#[derive(Alga)]
struct IntVec(i32, i32);

#[test]
fn int_fields_reach_ring_only() {
    let a = IntVec(1, 2);
    let b = IntVec(3, 4);
    let s = <IntVec as Magma<Additive>>::combine(&a, &b);
    assert_eq!(s.0, 4);
    fn assert_ring<T: Ring<Additive, Multiplicative>>() {}
    assert_ring::<IntVec>();
}
