# [Yahoo Finance](https://github.com/patchfx/yahoo-finance)

[![Travis Build Status](https://travis-ci.org/patchfx/yahoo-finance.svg?branch=master)](https://travis-ci.org/patchfx/yahoo-finance)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

```
extern crate yahoo_finance;

use yahoo_finance::stock::Stock;

fn main() {
    let mut stock = Stock { symbol: "GOOG".to_string(), current: "0.0".to_string() };
    stock.update_current_price();
    println!("GOOG Current Price: {}", stock.current);
}
```
