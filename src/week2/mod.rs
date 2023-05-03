struct Solution {}

impl Solution {
  fn format_string(words: Vec<String>, max_width: i32) -> Vec<String> { vec![] }
}

mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let words = vec![
      "TowerBFT".to_string(),
      "allows".to_string(),
      "the".to_string(),
      "Solana".to_string(),
      "network".to_string(),
      "to".to_string(),
      "process".to_string(),
      "thousands".to_string(),
      "of".to_string(),
      "transactions".to_string(),
      "per".to_string(),
      "second".to_string(),
      "and".to_string(),
      "is".to_string(),
      "easily".to_string(),
      "scalable".to_string(),
      "to".to_string(),
      "meet".to_string(),
      "the".to_string(),
      "growing".to_string(),
      "demands".to_string(),
      "of".to_string(),
      "the".to_string(),
      "blockchain".to_string(),
      "network.".to_string(),
    ];
    let max_width = 16;

    let expected = [
      "TowerBFT  allows",
      "the       Solana",
      "network       to",
      "process         ",
      "thousands     of",
      "transactions per",
      "second   and  is",
      "easily  scalable",
      "to    meet   the",
      "growing  demands",
      "of           the",
      "blockchain      ",
      "network.        ",
    ];
    assert_eq!(Solution::format_string(words, max_width), expected);
  }

  #[test]
  fn test_2() {
    let words = vec![
      "This".to_string(),
      "is".to_string(),
      "a".to_string(),
      "simple".to_string(),
      "test".to_string(),
      "case".to_string(),
      "for".to_string(),
      "center".to_string(),
      "justification.".to_string(),
    ];
    let max_width = 14;
    let expected = ["This    is   a", "simple    test", "case       for", "center        ", "justification."];
    assert_eq!(Solution::format_string(words, max_width), expected);
  }
}
