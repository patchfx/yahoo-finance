extern crate curl;
pub mod stock;

mod yahoo_finance { }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_update_a_stocks_current_price() {
    let mut stock = stock::Stock { symbol: "IBM".to_string(), current: "0.0".to_string() };
    stock.update_current_price();
    assert!(stock.current != "0.0");
  }
}
