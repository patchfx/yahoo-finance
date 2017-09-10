extern crate curl;
pub mod stock;

mod yahoo_finance { }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_update_a_stocks_current_price() {
    let mut stock = stock::Stock::new("GOOG");
    stock.update();
    assert!(stock.current != "0.0");
  }

  #[test]
  fn can_update_a_stocks_ask_price() {
    let mut stock = stock::Stock::new("GOOG");
    stock.update();
    assert!(stock.ask != "0.0");
  }

  #[test]
  fn can_update_a_stocks_bid_price() {
    let mut stock = stock::Stock::new("GOOG");
    stock.update();
    assert!(stock.bid != "0.0");
  }
}
