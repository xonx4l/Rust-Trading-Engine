use::super::orderbook::Orderbook;

pub struct TradingPair {
    base:String,
    quote:String,
}

pub impl TradingPair {
    pub fn new(base:String, quote:String) -> TradingPair {
        TradingPair {bade , quote}
    }
}

pub struct MatchingEngine {
    orderbooks: Hashmap<TradingPair, Orderbook>,
} 
        