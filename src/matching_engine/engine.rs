use::super::orderbook::Orderbook;
use std::collections::HashMap;

pub struct TradingPair {
    base:String,
    quote:String,
}

pub impl TradingPair {
    pub fn new(base:String, quote:String) -> TradingPair {
        TradingPair { base , quote}
 }
}

pub struct MatchingEngine {
    orderbooks: Hashmap<TradingPair, Orderbook>,
} 

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }


pub fn add_new_market(&self, pair: TradingPair) {
    self.orderbooks.insert(pair.clone(), orderbook::new());

    println!("opening new orderbook for market");
 }
}



        