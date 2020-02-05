use std::collections::HashMap;

mod holding;
mod constants;

pub trait Holdable {
    fn allowance(&self, id: &str) -> &u128;
    fn increase_allowance(&mut self, id: &str, amt: u128) -> Result<bool, &str>;
    fn decrease_allowance(&mut self, id: &str, amt: u128) -> Result<bool, &str>;
    fn locked(&self, id: &str) -> &u128;
    fn lock(&mut self, id: &str, amt: u128) -> Result<bool, &str>;
    fn unlock(&mut self, id: &str, amt: u128) -> Result<bool, &str>;
}

pub trait Fungible {
    fn total_supply(&self) -> u128;
    fn balance_of(&self) -> u128;
    fn transfer(&mut self, to: &str, amt: u128) -> bool;
    fn transfer_from(&mut self, from: &str, to: &str, amt: u128) -> bool;
}

pub struct Token {
    pub decimals: u8,
    pub supply: u128,
    pub holdings: HashMap<String, Box<dyn Holdable>>,
}

impl Token {
    #[allow(dead_code)]
    fn new() -> Self {
        let t = Self {
            decimals: 18,
            supply: 0,
            holdings: HashMap::new()
        };
        t
    }
}
