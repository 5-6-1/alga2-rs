//! Generic structs with a where-clause constraint (not inline bound): the
//! derive must splice the struct's where clause without duplicating `where`.
use alga2_derive::Alga;

#[derive(Alga)]
#[alga(level = "Group")]
struct Pair<T>
where
    T: Clone,
{
    x: T,
    y: T,
}

#[test]
fn where_clause_generic() {
    let a = Pair { x: 1i32, y: 2 };
    let b = Pair { x: 3, y: 4 };
    let s = <Pair<i32> as alga2::tower::Magma<alga2::op::Additive>>::combine(&a, &b);
    assert_eq!(s.x, 4);
}
