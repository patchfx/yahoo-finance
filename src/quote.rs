const YAHOO_FINANCE_URL: &'static str = "http://download.finance.yahoo.com/d/quotes.csv?s=";
use curl::easy::Easy;
use tags::Tags;

pub struct Quote {
  pub symbol: String,
  pub current: String,
  pub bid: String,
  pub bid_realtime: String,
  pub ask: String,
  pub ask_realtime: String,
  pub previous_close: String,
  pub open: String,
}

impl Quote {
  pub fn new(symbol: &str) -> Quote {
    Quote {
      symbol: symbol.to_string(),
      current: "N/A".to_string(),
      bid: "N/A".to_string(),
      bid_realtime: "N/A".to_string(),
      ask: "N/A".to_string(),
      ask_realtime: "N/A".to_string(),
      previous_close: "N/A".to_string(),
      open: "N/A".to_string(),
    }
  }

  pub fn update(&mut self) {
    let mut data = Vec::new();
    let mut curl = Easy::new();

    let tags = Tags::new();
    let url = format!("{}{}&f={}", YAHOO_FINANCE_URL, self.symbol, tags.pricing.join(""));

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

    self.ask = res[0].clone();
    self.bid = res[1].clone();
    self.ask_realtime = res[2].clone();
    self.bid_realtime = res[3].clone();
    self.previous_close = res[4].clone();
    self.open = res[5].clone();
    self.current = res[6].clone();
  }
}
