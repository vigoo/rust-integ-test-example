use rust_integ_test::function_to_test;
use crate::SHARED;

#[test]
fn second_tests_a() {
    assert_eq!(function_to_test(), 42)
}

#[test]
fn second_tests_b() {
    assert_eq!(SHARED.read().get_something(), 11)
}