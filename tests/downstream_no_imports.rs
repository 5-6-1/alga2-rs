//! Downstream-mimicking check: a user imports ONLY the derive macro and
//! writes one line — no tower trait imports. This is the crate's promise.
use alga2_derive::Alga;

#[derive(Alga)]
#[alga(level = "Field")]
struct Vec2 {
    x: f64,
    y: f64,
}

#[test]
fn no_trait_imports_needed() {
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    let s = <Vec2 as alga2::tower::Magma<alga2::op::Additive>>::combine(&a, &b);
    assert_eq!(s.x, 4.0);
    let inv = <Vec2 as alga2::tower::DivisionRing<
        alga2::op::Additive,
        alga2::op::Multiplicative,
    >>::inv(&a);
    assert_eq!(inv.y, 0.5);
    let g = <Vec2 as alga2::tower::Group<alga2::op::Additive>>::inverse(&a);
    assert_eq!(g.y, -2.0);
}
