use::super::orderbook::{Order,Orderbook};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base:String,
    quote:String,
}

impl TradingPair {
    pub fn new(base:String, quote:String) -> TradingPair {
        TradingPair { base , quote}
 }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
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

    println!("opening new orderbook for market {:?}", pair.to_string());
   }


pub fn place_limit_order(&mut self, pair: TradingPair, price:f64, order:Order) -> Result<(), String> {
              match self.orderbook.get_mut(&pair) {
                   Some(orderbook) => {
                       orderbook.add_order(price, order);

                       println!(" placed limit order at price level {}", price);

                       Ok(())
                   }
                   None => Err(format!(
                    "the orderbook for the given trading pair ({}) does not exist", 
                    pair.to_string()
                   )),
              }
    }
}


        