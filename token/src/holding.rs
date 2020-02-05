use std::collections::{hash_map::Entry, HashMap};
use crate::{Holdable, constants};

pub struct Holding {
    pub allowances: HashMap<String, u128>,
    pub locks: HashMap<String, u128>,
    pub supply: u128,
}

impl Holding {
    #[allow(dead_code)]
    fn new(amt: u128) -> Self {
        let h = Self {
            allowances: HashMap::new(),
            locks: HashMap::new(),
            supply: amt,
        };
        h
    }

    /// A Holding's Total Supply is `supply` + the sum of its `locks`.
    #[allow(dead_code)]
    fn total_supply(&self) -> u128 {
        self.supply + self.locks.values().sum::<u128>()
    }
}

impl Holdable for Holding {
    /// An amount of our supply a given spender may spend.
    /// # Arguments
    /// * `id` - A string slice corresponding to a Near account id.
    /// # Returns
    /// * Pointer to the current allowed amount. Defaults to 0 if absent.
    fn allowance(&self, id: &str) -> &u128 {
        self.allowances.get(id).unwrap_or(&0)
    }

    /// Raise the allowed entry by the given amount.
    /// # Arguments
    /// * `id` - A string slice corresponding to a Near account id.
    /// * `amt` - A u128 value (not reference) to increase the allowance by.
    /// # Returns
    /// * Result<bool, &str>: true if successful. Err string otherwise.
    /// NOTE: This method will also initialize a new entry if none was present.
    fn increase_allowance(&mut self, id: &str, amt: u128) -> Result<bool, &str> {
        let a = self.allowances.entry(id.to_string()).or_insert(0);
        match a.checked_add(amt) {
            None => Err(constants::WOULD_OVERFLOW),
            Some(i) => {
                *a = i;
                Ok(true)
            }
        }
    }

    /// Lower the allowed entry by the given amount if it exists and if the operation
    /// would not lower an existing allowance below an existing lock.
    /// # Arguments
    /// * `id` - A string slice corresponding to a Near account id.
    /// * `amt` - A u128 value (not reference) to decrease the allowance by.
    /// # Returns
    /// * Result<bool, &str>: true if successful. Err string otherwise.
    fn decrease_allowance(&mut self, id: &str, amt: u128) -> Result<bool, &str> {
        match self.allowances.entry(id.to_string()) {
            Entry::Occupied(mut e) => {
                match e.get().checked_sub(amt) {
                    None => Err(constants::WOULD_UNDERFLOW),
                    Some(i) => {
                        // NOTE: cannot use locked here as self is mutable in this scope
                        if self.locks.get(id).unwrap_or(&0) > &i {
                            Err(constants::ALLOWANCE_LESS_THAN_LOCK)
                        } else {
                            *e.get_mut() = i;
                            Ok(true)
                        }
                    }
                }
            },
            _ => Err(constants::UNOCCUPIED_ENTRY),
        }
    }

    /// The amount currently held for a given identifier
    fn locked(&self, id: &str) -> &u128 {
        self.locks.get(id).unwrap_or(&0)
    }

    /// Given an id and an amount for said id enforce these invariants:
    /// * allowance exists and it is >= aggregate lock
    /// * current supply can be reduced by this amount
    /// With invariants met, add this amount to an existing lock, or initialize a new one.
    /// Note that lock cannot alter allowance.
    fn lock(&mut self, id: &str, amt: u128) -> Result<bool, &str> {
        let locked = self.locks.entry(id.to_string()).or_insert(0);
        let mut err_msg = "";

        match locked.checked_add(amt) {
            None => Err(constants::WOULD_OVERFLOW),
            Some(n) => {
                // allowance. note you are comparing to the aggregate n
                if let Entry::Occupied(e) = self.allowances.entry(id.to_string()) {
                    if e.get() < &n {
                        err_msg = constants::EXCEEDS_ALLOWANCE;
                    }
                } else {
                    err_msg = constants::NO_ALLOWANCE;
                }
                // supply. note that you are subtracting amt, not the aggregate n
                if let Some(s) = self.supply.checked_sub(amt) {
                    self.supply = s;
                } else {
                    err_msg = constants::LOCK_GREATER_THAN_SUPPLY;
                }

                if err_msg != "" {
                    self.locks.remove_entry(&id.to_string());
                    return Err(err_msg);
                }
                // invariants enforced, set the new locked amount
                *locked = n;
                Ok(true)
            }
        }
    }

    /// Given an amount `lte` a locked amount, subtract it from a lock if present.
    fn unlock(&mut self, id: &str, amt: u128) -> Result<bool, &str> {
        match self.locks.entry(id.to_string()) {
            Entry::Occupied(mut e) => {
                match e.get().checked_sub(amt) {
                    None => Err(constants::WOULD_UNDERFLOW),
                    Some(i) => {
                        *e.get_mut() = i;
                        Ok(true)
                    }
                }
            },
            _ => Err(constants::UNOCCUPIED_ENTRY),
        }
    }
}

#[cfg(test)]
#[path = "./holding_tests.rs"]
mod tests;
