# [Yahoo Finance](https://github.com/patchfx/yahoo-finance)

[![Travis Build Status](https://travis-ci.org/patchfx/yahoo-finance.svg?branch=master)](https://travis-ci.org/patchfx/yahoo-finance)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

```
extern crate yahoo_finance;

use yahoo_finance::quote::Quote;

fn main() {
    let mut stock = Quote::new("GOOG");
    stock.update();
    println!("GOOG Current Price: {}", stock.current);
    println!("GOOG Bid Price: {}", stock.bid);
    println!("GOOG Ask Price: {}", stock.ask);
}
```
