const YAHOO_FINANCE_URL: &'static str = "http://download.finance.yahoo.com/d/quotes.csv?s=";
use curl::easy::Easy;

pub struct Stock {
  pub symbol: String,
  pub current: String,
  pub bid: String,
  pub ask: String,
}

impl Stock {
  pub fn new(symbol: &str) -> Stock {
    Stock {
      symbol: symbol.to_string(),
      current: "0.0".to_string(),
      bid: "0.0".to_string(),
      ask: "0.0".to_string(),
    }
  }

  pub fn update(&mut self) {
    let mut data = Vec::new();
    let mut curl = Easy::new();
    let url = format!("{}{}&f=l1ab", YAHOO_FINANCE_URL, self.symbol);
    curl.url(&url).unwrap();
    {
      let mut transfer = curl.transfer();
      transfer.write_function(|d| {
        data.extend_from_slice(d);
        Ok(d.len())
      }).unwrap();
      transfer.perform().unwrap()
    }
    let prices = String::from_utf8(data).unwrap();
    let res: Vec<String> = prices.split(",").map(|s| s.to_string()).collect();
    self.current = res[0].clone();
    self.ask = res[0].clone();
    self.bid = res[2].clone();
  }
}
