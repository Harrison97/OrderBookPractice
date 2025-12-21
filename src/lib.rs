use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq)]
struct Order {
    id: i64,
    price: i64,
    qty: i64,
}
impl Order {
    fn new(id: i64, price: i64, qty: i64) -> Self {
        Self { id, price, qty }
    }
    fn validate(&self) -> Result<(), String> {
        if self.price <= 0 {
            Err("Order price must be > 0.".to_string())
        } else if self.qty <= 0 {
            Err("Order qty must be > 0.".to_string())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct OrderBook {
    num_submitted: i64,
    buy_side: BTreeMap<i64, VecDeque<Order>>,
    sell_side: BTreeMap<i64, VecDeque<Order>>,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            num_submitted: 0,
            buy_side: BTreeMap::<i64, VecDeque<Order>>::new(),
            sell_side: BTreeMap::<i64, VecDeque<Order>>::new(),
        }
    }
    pub fn submit(&mut self, side: Side, price: i64, qty: i64) -> Result<(), String> {
        let order = Order::new(self.num_submitted, price, qty);
        order.validate()?;
        let book_side = match side {
            Side::Buy => &mut self.buy_side,
            Side::Sell => &mut self.sell_side,
        };
        book_side
            .entry(price)
            .or_insert_with(VecDeque::<Order>::new)
            .push_back(order);
        self.num_submitted += 1;
        println!(
            "Submitted {:?} order at price {} for qty {}.",
            side, price, qty
        );
        while self.fill_next() {}
        Ok(())
    }
    pub fn fill_next(&mut self) -> bool {
        match (self.sell_side.first_entry(), self.buy_side.last_entry()) {
            (Some(mut lowest_asks_entry), Some(mut highest_bids_entry)) => {
                if lowest_asks_entry.key() > highest_bids_entry.key() {
                    return false;
                }
                let lowest_asks = lowest_asks_entry.get_mut();
                let highest_bids = highest_bids_entry.get_mut();
                let Some(first_ask) = lowest_asks.front_mut() else {
                    self.sell_side.pop_first();
                    return false;
                };
                let Some(first_bid) = highest_bids.front_mut() else {
                    self.buy_side.pop_last();
                    return false;
                };

                let price = std::cmp::min_by_key(&first_ask, &first_bid, |o| o.id).price;
                let qty = first_ask.qty.min(first_bid.qty);

                first_ask.qty -= qty;
                first_bid.qty -= qty;
                if first_ask.qty == 0 {
                    lowest_asks.pop_front();
                    if lowest_asks.is_empty() {
                        self.sell_side.pop_first();
                    }
                }
                if first_bid.qty == 0 {
                    highest_bids.pop_front();
                    if highest_bids.is_empty() {
                        self.buy_side.pop_last();
                    }
                }
                println!("Filled order at price {} for qty {}.", price, qty);
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        let order = Order::new(1, 2, 3);
        println!("{:?}", order);
        assert!(
            order
                == Order {
                    id: 1,
                    price: 2,
                    qty: 3
                }
        );
        assert!(order.validate().is_ok());

        let order = Order::new(1, -2, 3);
        println!("{:?}", order);
        assert!(order.validate() == Err("Order price must be > 0.".to_string()));

        let order = Order::new(1, 2, -3);
        println!("{:?}", order);
        assert!(order.validate() == Err("Order qty must be > 0.".to_string()));
    }

    #[test]
    fn test_order_book() {
        let book = OrderBook::new();
        println!("{:?}", book);
    }
}
