use crate::holding::Holding;
use std::collections::HashMap;

mod constants;
mod holding;

pub trait Holdable {
    fn allowance(&self, spender: &str) -> &u128;
    fn increase_allowance(&mut self, spender: &str, amt: u128) -> Result<bool, &str>;
    fn decrease_allowance(&mut self, spender: &str, amt: u128) -> Result<bool, &str>;
    fn locked(&self, spender: &str) -> &u128;
    fn lock(&mut self, spender: &str, amt: u128) -> Result<bool, &str>;
    fn unlock(&mut self, spender: &str, amt: u128) -> Result<bool, &str>;
}

pub trait Fungible {
    fn decimals(&self) -> &u8;
    // TODO: This should likely be a u256 when supported as the holdings can hold u128 each
    fn total_supply(&self) -> u128;
    fn balance_of(&self, owner: &str) -> &u128;
    fn allowance(&self, owner: &str, spender: &str) -> &u128;
    // fn approve(&mut self, spender: &str, amt: u128) -> bool;
    // fn transfer(&mut self, to: &str, amt: u128) -> bool;
    // fn transfer_from(&mut self, from: &str, to: &str, amt: u128) -> bool;
}

pub struct Token {
    pub decimals: u8,
    pub supply: u128,
    // TODO use the near Map
    pub holdings: HashMap<String, Holding>,
}

impl Token {
    #[allow(dead_code)]
    fn new(id: &str, amt: u128) -> Self {
        let mut t = Self {
            // yoctoNear
            decimals: 24,
            supply: amt,
            // TODO use the near Map
            holdings: HashMap::new(),
        };
        t.holdings.insert(id.to_string(), Holding::new(amt));
        t
    }
}

impl Fungible for Token {
    fn decimals(&self) -> &u8 {
        &self.decimals
    }

    fn total_supply(&self) -> u128 {
        self.holdings.values().fold(0, |s, h| s + h.supply)
    }

    fn balance_of(&self, owner: &str) -> &u128 {
        if let Some(h) = self.holdings.get(owner) {
            &h.supply
        } else {
            &0
        }
    }

    fn allowance(&self, owner: &str, spender: &str) -> &u128 {
        if let Some(h) = self.holdings.get(owner) {
            h.allowance(spender)
        } else {
            &0
        }
    }
}
