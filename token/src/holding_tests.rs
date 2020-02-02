#[cfg(test)]
use crate::Holdable;
use super::Holding;

#[test]
fn no_allowance_yet() {
    let h = Holding::new();
    assert_eq!(h.allowance("foo"), &0);
}

#[test]
fn no_lock_yet() {
    let h = Holding::new();
    assert_eq!(h.locked("bar"), &0);
}

#[test]
fn sets_initial_allowance() {
    let mut h = Holding::new();
    let a: u128 = 1000000000;
    let ok = h.increase_allowance("foo", a);
    assert_eq!(ok, true);
    assert_eq!(h.allowance("foo"), &a);
}

#[test]
fn sets_initial_lock() {
    let mut h = Holding::new();
    let a: u128 = 1000000000;
    let ok = h.lock("bar", a);
    assert_eq!(ok, true);
    assert_eq!(h.locked("bar"), &a);
}

#[test]
fn increase_allowance() {
    let mut h = Holding::new();
    // apparently cargo is able to cast this int literal for us
    let ok = h.increase_allowance("foo", 100000);
    assert_eq!(ok, true);
    let a: u128 = 50000;
    let ok = h.increase_allowance("foo", a);
    assert_eq!(ok, true);
    assert_eq!(h.allowance("foo"), &(a*3));
}

#[test]
fn lock_is_additive() {
    let mut h = Holding::new();
    // apparently cargo is able to cast this int literal for us
    let ok = h.lock("bar", 100000);
    assert_eq!(ok, true);
    let a: u128 = 50000;
    let ok = h.lock("bar", a);
    assert_eq!(ok, true);
    assert_eq!(h.locked("bar"), &(a*3));
}

#[test]
fn unoccupied_allowance_is_noop() {
    let mut h = Holding::new();
    let ok = h.decrease_allowance("bar", 1000000);
    assert_eq!(ok, false);
}

#[test]
fn unoccupied_lock_is_noop() {
    let mut h = Holding::new();
    let ok = h.unlock("baz", 1000000);
    assert_eq!(ok, false);
}

#[test]
fn cant_decrease_gt_allowance() {
    let mut h = Holding::new();
    let ok = h.increase_allowance("foo", 1000000);
    assert_eq!(ok, true);
    let ok = h.decrease_allowance("foo", 2000000);
    assert_eq!(ok, false);
}

#[test]
fn cant_decrease_gt_lock() {
    let mut h = Holding::new();
    let ok = h.lock("bar", 1000000);
    assert_eq!(ok, true);
    let ok = h.unlock("bar", 2000000);
    assert_eq!(ok, false);
}

#[test]
fn decrease_allowance() {
    let mut h = Holding::new();
    let ok = h.increase_allowance("foo", 100000);
    assert_eq!(ok, true);
    let a: u128 = 50000;
    let ok = h.decrease_allowance("foo", a);
    assert_eq!(ok, true);
    assert_eq!(h.allowance("foo"), &a);
}

#[test]
fn unlock() {
    let mut h = Holding::new();
    let ok = h.lock("bar", 100000);
    assert_eq!(ok, true);
    let a: u128 = 50000;
    let ok = h.unlock("bar", a);
    assert_eq!(ok, true);
    assert_eq!(h.locked("bar"), &a);
}
