use std::collections::{hash_map::Entry, HashMap};

pub trait Holdable {
    fn allowance(&self, id: &str) -> &u128;
    fn increase_allowance(&mut self, id: &str, amt: u128) -> bool;
    fn decrease_allowance(&mut self, id: &str, amt: u128) -> bool;
    fn held(&self, id: &str) -> &u128;
    fn increase_hold(&mut self, id: &str, amt: u128) -> bool;
    fn decrease_hold(&mut self, id: &str, amt: u128) -> bool;
}

pub struct Holding {
    pub allowances: HashMap<String, u128>,
    pub holds: HashMap<String, u128>,
    pub supply: u128,
}

impl Holding {
    fn new() -> Self {
        let h = Self {
            allowances: HashMap::new(),
            holds: HashMap::new(),
            supply: 0,
        };
        h
    }
}

impl Holdable for Holding {
    /// An amount of our supply allocated for a given spender.
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

    fn held(&self, id: &str) -> &u128 {
        self.holds.get(id).unwrap_or(&0)
    }

    fn increase_hold(&mut self, id: &str, amt: u128) -> bool {
        let a = self.holds.entry(id.to_string()).or_insert(0);
        *a += amt;
        true
    }

    fn decrease_hold(&mut self, id: &str, amt: u128) -> bool {
        if let Entry::Occupied(mut e) = self.holds.entry(id.to_string()) {
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
