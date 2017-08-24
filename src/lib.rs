extern crate curl;

mod yahoo_finance {
  const YAHOO_FINANCE_URL: &'static str = "http://download.finance.yahoo.com/d/quotes.csv?s=";
  use curl::easy::Easy;

  pub struct Stock {
      pub symbol: String,
      pub current: String,
  }

  impl Stock {
      pub fn update_current_price(&mut self) {
        let mut data = Vec::new();
        let mut curl = Easy::new();
        let url = format!("{}{}&f=l1", YAHOO_FINANCE_URL, self.symbol);
        curl.url(&url).unwrap();
        {
          let mut transfer = curl.transfer();
          transfer.write_function(|d| {
            data.extend_from_slice(d);
            Ok(d.len())
          }).unwrap();
          transfer.perform().unwrap()
        }
        self.current = String::from_utf8(data).unwrap();
      }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_update_a_stocks_current_price() {
      let mut stock = yahoo_finance::Stock { symbol: "IBM".to_string(), current: "0.0".to_string() };
      stock.update_current_price();
      assert!(stock.current != "0.0");
    }
}
