use std::collections::HashMap;

mod holding;

pub trait Holdable {
    fn allowance(&self, id: &str) -> &u128;
    fn increase_allowance(&mut self, id: &str, amt: u128) -> bool;
    fn decrease_allowance(&mut self, id: &str, amt: u128) -> bool;
    fn locked(&self, id: &str) -> &u128;
    fn lock(&mut self, id: &str, amt: u128) -> bool;
    fn unlock(&mut self, id: &str, amt: u128) -> bool;
}

pub trait Fungible {
    fn transfer_from(&mut self, from: &str, to: &str, amt: u128) -> bool;
    fn transfer(&mut self, to: &str, amt: u128) -> bool;
    fn total_supply(&self) -> u128;
}

pub struct Token {
    pub supply: u128,
    pub holdings: HashMap<String, Box<dyn Holdable>>,
}

impl Token {
    fn new() -> Self {
        let t = Self {
            supply: 0,
            holdings: HashMap::new()
        };
        t
    }
}
