//! Generic structs: the derive must forward the struct's own generics into
//! the impls' where clauses (`impl<T: Clone> Group<T> for Pair<T>`).
use alga2_derive::Alga;

#[derive(Alga)]
#[alga(level = "Group")]
struct Pair<T: Clone> {
    x: T,
    y: T,
}

#[test]
fn generic_pair() {
    let a = Pair { x: 1i32, y: 2 };
    let b = Pair { x: 3, y: 4 };
    let s = <Pair<i32> as alga2::tower::Magma<alga2::op::Additive>>::combine(&a, &b);
    assert_eq!(s.x, 4);
    let inv = <Pair<i32> as alga2::tower::Group<alga2::op::Additive>>::inverse(&a);
    assert_eq!(inv.y, -2);
}
