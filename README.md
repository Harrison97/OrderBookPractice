# OrderBookPractice

A single-threaded limit order book and matching engine in Rust. I wrote it to practice Rust fundamentals on a problem I know from trading: price-time priority matching with partial fills.

## How it works

The book keeps each side in a `BTreeMap<i64, VecDeque<Order>>`:

- **Price levels are sorted.** The best ask is `sell_side.first_entry()` and the best bid is `buy_side.last_entry()`. Both are O(log n), and no separate heap or index is needed.
- **Each level is a FIFO queue.** Orders at the same price fill in arrival order, which gives time priority.
- **Matching runs on every submit.** `submit` validates the order, rests it on its side, then calls `fill_next` until the book no longer crosses (best ask > best bid).
- **Trades print at the resting order's price.** Each order gets an increasing id, and the fill price comes from the older of the two orders. An aggressive buy at 22 against a resting ask at 20 fills at 20.
- **Partial fills.** Each fill takes `min(ask.qty, bid.qty)`. A filled order leaves its queue, and an empty price level leaves the map.
- **Validation.** Orders with a non-positive price or quantity are rejected with an `Err` and never reach the book.

Prices are integer ticks (`i64`), so matching never compares floats.

## Run it

```bash
cargo run    # replays a short scripted session and dumps the final book
cargo test
```

```text
Submitted Buy order at price 21 for qty 3.
Submitted Sell order at price 20 for qty 4.
Filled order at price 21 for qty 3.
Submitted Buy order at price 19 for qty 3.
Submitted Buy order at price 22 for qty 3.
Filled order at price 20 for qty 1.
```

## Scope

I kept the scope narrow on purpose: single-threaded, synchronous, no `unsafe`, no dependencies. There are only limit orders. Cancels, modifies, market orders and order types such as IOC/FOK aren't implemented, and fills are printed rather than returned as events.
