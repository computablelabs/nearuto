use std::collections::HashMap;
use crate::holding::{Holdable, Holding};

mod holding;

pub trait Fungible {

}

pub struct Token {
    pub holdings: HashMap<String, Box<dyn Holdable>>,
}

impl Token {
    fn new() -> Self {
        let t = Self {
            holdings: HashMap::new()
        };
        t
    }
}
