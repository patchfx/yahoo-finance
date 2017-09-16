extern crate curl;

pub mod quote;
pub mod tags;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_update_a_stocks_current_price() {
    let mut stock = quote::Quote::new("GOOG");
    assert!(stock.current == "N/A");
    stock.update();
    assert!(stock.current != "N/A");
  }

  #[test]
  fn can_update_a_stocks_ask_price() {
    let mut stock = quote::Quote::new("GOOG");
    assert!(stock.current == "N/A");
    stock.update();
    assert!(stock.ask != "N/A");
  }

  #[test]
  fn can_update_a_stocks_bid_price() {
    let mut stock = quote::Quote::new("GOOG");
    assert!(stock.current == "N/A");
    stock.update();
    assert!(stock.bid != "N/A");
  }
}
