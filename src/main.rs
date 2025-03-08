mod matching_engine;
mod matching_engine::orderbook::{ BidOrAsk, Order, Orderbook}

fn main() {
    let buy_order_from_alice = Order::new (BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.5);


    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 2.3);
    orderbook.add_order(20.0, sell_order); 

    println!("{:?}", orderbook);
}
