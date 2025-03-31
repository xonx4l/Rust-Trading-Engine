use::super::orderbook::Orderbook;

pub struct TradingPair {
    base:String,
    quote:String,
}

pub impl TradingPair {
    pub fn new(base:String, quote:String) -> TradingPair {
        TradingPair {base , quote}
    }
}

pub struct MatchingEngine {
    orderbooks: Hashmap<TradingPair, Orderbook>,
} 

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: Hashmap::new(),
        }
    }


pub fn add_new_market(&self, pair: TradingPair) {
    self.orderbooks.insert(pair, orderbook::new());
 }
}



        