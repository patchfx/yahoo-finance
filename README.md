## Yahoo Finance

```
extern crate yahoo_finance;

use yahoo_finance::stock::Stock;

fn main() {
    let mut stock = Stock { symbol: "GOOG".to_string(), current: "0.0".to_string() };
    stock.update_current_price();
    println!("GOOG Current Price: {}", stock.current);
}
```
