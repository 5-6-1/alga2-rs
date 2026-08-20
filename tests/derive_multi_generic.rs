//! Two generic parameters and mixed generic/concrete fields: bounds must
//! deduplicate by distinct type and forward all params correctly.
use alga2_derive::Alga;

#[derive(Alga, PartialEq, Debug)]
#[alga(level = "Group")]
struct TwoParams<A, B> {
    x: A,
    y: B,
}

#[derive(Alga, PartialEq, Debug)]
#[alga(level = "Group")]
struct Mixed<T> {
    x: T,
    y: i32,
}

#[test]
fn two_params() {
    let a = TwoParams { x: 1i32, y: 2i64 };
    let b = TwoParams { x: 3, y: 4 };
    let s = <TwoParams<i32, i64> as alga2::tower::Magma<alga2::op::Additive>>::combine(&a, &b);
    assert_eq!(s, TwoParams { x: 4, y: 6 });
    let inv = <TwoParams<i32, i64> as alga2::tower::Group<alga2::op::Additive>>::inverse(&a);
    assert_eq!(inv, TwoParams { x: -1, y: -2 });
}

#[test]
fn mixed_generic_concrete() {
    let a = Mixed { x: 1i32, y: 2 };
    let b = Mixed { x: 3, y: 4 };
    let s = <Mixed<i32> as alga2::tower::Magma<alga2::op::Additive>>::combine(&a, &b);
    assert_eq!(s, Mixed { x: 4, y: 6 });
}
