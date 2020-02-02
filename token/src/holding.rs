use std::collections::{hash_map::Entry, HashMap};
use crate::Holdable;

pub struct Holding {
    pub allowances: HashMap<String, u128>,
    pub locks: HashMap<String, u128>,
    pub supply: u128,
}

impl Holding {
    fn new() -> Self {
        let h = Self {
            allowances: HashMap::new(),
            locks: HashMap::new(),
            supply: 0,
        };
        h
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
    /// * `true`
    /// NOTE: This method will also initialize a new entry if none was present.
    fn increase_allowance(&mut self, id: &str, amt: u128) -> bool {
        let a = self.allowances.entry(id.to_string()).or_insert(0);
        *a += amt;
        true
    }

    /// Lower the allowed entry by the given amount.
    /// # Arguments
    /// * `id` - A string slice corresponding to a Near account id.
    /// * `amt` - A u128 value (not reference) to decrease the allowance by.
    /// # Returns
    /// * Boolean. `true` if successful. `false` otherwise.
    /// NOTE: If an attempt is made to decrease more than is available, we return `false`.
    /// TODO: Rather than the above, we could return an Error or panic.
    fn decrease_allowance(&mut self, id: &str, amt: u128) -> bool {
        if let Entry::Occupied(mut e) = self.allowances.entry(id.to_string()) {
            if e.get() >= &amt {
                *e.get_mut() -= amt;
                return true;
            }
        }
        false
    }

    /// The amount currently held for a given identifier
    fn locked(&self, id: &str) -> &u128 {
        self.locks.get(id).unwrap_or(&0)
    }

    /// Given an amount either initialize a new lock if none present,
    /// or add said amount to an existing one.
    fn lock(&mut self, id: &str, amt: u128) -> bool {
        let a = self.locks.entry(id.to_string()).or_insert(0);
        *a += amt;
        true
    }

    /// Given an amount `lte` a locked amount, subtract it from a lock if present.
    fn unlock(&mut self, id: &str, amt: u128) -> bool {
        if let Entry::Occupied(mut e) = self.locks.entry(id.to_string()) {
            if e.get() >= &amt {
                *e.get_mut() -= amt;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
#[path = "./holding_tests.rs"]
mod tests;
