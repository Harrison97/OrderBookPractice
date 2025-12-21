/*
### Requirements

- `Order` struct
- `Side` enum
- Order book with:
    - Buy side
    - Sell side
- Price-time priority (simplified)
- Partial fills
- Deterministic matching loop

### Constraints

- Single-threaded
- No async
- No unsafe
- Explain every design choice

### What This Proves

- You understand Rust fundamentals
- You can structure real systems
- You can explain correctness + tradeoffs
*/

use matching_engine::{OrderBook, Side};

fn main() {
    println!("Hello, world!");
    let mut book = OrderBook::new();
    book.submit(Side::Buy, 21, 3);
    book.submit(Side::Sell, 20, 4);
    book.submit(Side::Buy, 19, 3);
    book.submit(Side::Buy, 22, 3);
    book.submit(Side::Buy, -22, 3);
    dbg!(&book);
}
