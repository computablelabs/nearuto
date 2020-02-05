#[cfg(test)]
use crate::Holdable;
use super::Holding;

#[test]
fn no_allowance_yet() {
    let h = Holding::new(0);
    assert_eq!(h.allowance("foo"), &0);
}

#[test]
fn no_lock_yet() {
    let h = Holding::new(0);
    assert_eq!(h.locked("bar"), &0);
}

#[test]
fn sets_initial_allowance() {
    let mut h = Holding::new(0);
    let a: u128 = 1000000000;
    assert!(h.increase_allowance("foo", a).is_ok());
    assert_eq!(h.allowance("foo"), &a);
}

#[test]
fn sets_initial_lock() {
    let a: u128 = 1000000;
    let mut h = Holding::new(a);
    assert_eq!(h.supply, a);
    assert_eq!(h.total_supply(), a);
    assert!(h.increase_allowance("foo", a/2).is_ok());
    assert!(h.lock("foo", a/2).is_ok());
    assert_eq!(h.locked("foo"), &(a/2));
}

#[test]
fn increase_allowance_is_additive() {
    let mut h = Holding::new(0);
    // apparently cargo is able to cast this int literal for us
    assert!(h.increase_allowance("foo", 100000).is_ok());
    let a: u128 = 50000;
    assert!(h.increase_allowance("foo", a).is_ok());
    assert_eq!(h.allowance("foo"), &(a*3));
}

#[test]
fn increase_overflow_is_err() {
    let mut h = Holding::new(0);
    let a: u128 = 50000;
    assert!(h.increase_allowance("foo", a).is_ok());
    assert!(h.increase_allowance("foo", u128::max_value()).is_err());
    assert_eq!(h.allowance("foo"), &a);
}

#[test]
fn lock_is_additive() {
    let a: u128 = 1500000;
    let mut h = Holding::new(a);
    assert!(h.increase_allowance("bar", 1000000).is_ok());
    assert!(h.lock("bar", a/3).is_ok());
    assert_eq!(h.locked("bar"), &(a/3));
    assert!(h.lock("bar", a/3).is_ok());
    assert_eq!(h.locked("bar"), &(1000000));
}

#[test]
fn unoccupied_decrease_allowance_is_err() {
    let mut h = Holding::new(0);
    assert!(h.decrease_allowance("bar", 1000000).is_err());
}

#[test]
fn unoccupied_unlock_is_err() {
    let mut h = Holding::new(0);
    assert!(h.unlock("baz", 1000000).is_err());
}

#[test]
fn decrease_allowance_will_not_underflow() {
    let mut h = Holding::new(0);
    assert!(h.increase_allowance("foo", 1000000).is_ok());
    assert!(h.decrease_allowance("foo", 2000000).is_err());
}

#[test]
fn unlock_will_not_underflow() {
    let a: u128 = 1000000;
    let mut h = Holding::new(a);
    assert!(h.increase_allowance("foo", a).is_ok());
    assert!(h.lock("foo", a).is_ok());
    assert!(h.unlock("foo", a*2).is_err());
}

#[test]
fn decrease_allowance_is_subtractive() {
    let mut h = Holding::new(0);
    assert!(h.increase_allowance("foo", 100000).is_ok());
    let a: u128 = 50000;
    assert!(h.decrease_allowance("foo", a).is_ok());
    assert_eq!(h.allowance("foo"), &a);
}

#[test]
fn decrease_allowance_below_lock_is_err() {
    let mut h = Holding::new(0);
    let a: u128 = 100000;
    assert!(h.increase_allowance("foo", a).is_ok());
    assert!(h.decrease_allowance("foo", a+1).is_err());
    assert_eq!(h.allowance("foo"), &a);
}

#[test]
fn unlock_is_subtractive() {
    let mut h = Holding::new(1000000);
    assert!(h.increase_allowance("bar", 100000).is_ok());
    assert!(h.lock("bar", 100000).is_ok());
    let a: u128 = 50000;
    assert!(h.unlock("bar", a).is_ok());
    assert_eq!(h.locked("bar"), &a);
}
