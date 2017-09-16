pub struct Tags {
  pub pricing: Vec<String>,
}

impl Tags {
  pub fn new() -> Tags {
    let pricing = vec![
      "a".to_string(),
      "b".to_string(),
      "b2".to_string(),
      "b3".to_string(),
      "p".to_string(),
      "o".to_string(),
      "l1".to_string(),
    ];

    Tags {
      pricing: pricing
    }
  }
}
