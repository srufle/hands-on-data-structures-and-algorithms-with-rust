use example_simple::*;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
// #[should_panic]
fn add_1_2() {
    // 1 + 2 should = 3
    assert_eq!(add(1, 2), 3);
}

#[test]
fn add_1_1() {
    assert_eq!(add(1, 1), 2);
}
