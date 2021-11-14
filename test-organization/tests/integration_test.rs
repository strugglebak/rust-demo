mod adder;
mod common;
use crate::adder::add_two;
// cargo test --test integration_test
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}
