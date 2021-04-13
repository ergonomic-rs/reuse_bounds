//@TODO for `contain`- at the crate's top level:
//#![feature(custom_inner_attributes)]
// #![reuse_bounds_derive::accept_where_predicates_with_flexible_commas(x: y,, a:b)]

use reuse_bounds_macros::accept_where_predicates_with_flexible_commas;

#[accept_where_predicates_with_flexible_commas(x: y,, a:b)]
struct S {
    field: usize
}

#[accept_where_predicates_with_flexible_commas(,,)]
struct Empty;

#[test]
fn test_accept_where_predicates_with_flexible_commas() {
}