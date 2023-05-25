struct Solution_GaDo {}

impl Solution_GaDo {
  fn format_string(words: Vec<String>, max_width: i32) -> Vec<String> { 
    let mut result = Vec::new();
    let mut start_word = 0;
    while start_word < words.len() {
      let mut line_len = words[start_word].len();
      let mut end_word = start_word + 1;
      while end_word < words.len() && line_len + words[end_word].len() + (end_word - start_word) <= max_width as usize {
          line_len += words[end_word].len();
          end_word += 1;
      }
      let mut line = String::with_capacity(max_width as usize);
      let mut num_spaces = max_width as usize - line_len;
      let mut word_index = start_word;
      while word_index < end_word {
        line.push_str(&words[word_index]);
        if num_spaces > 0 {
            let spaces_between_words = if end_word == words.len() {
                if end_word - word_index == 1 { 
                    num_spaces 
                } 
                else { 
                    1 
                }
            } 
            else {
                if end_word - word_index - 1 > 0 {
                    (num_spaces - 1) / (end_word - word_index - 1) + 1
                } else {
                    num_spaces
                }
            };
            line.push_str(&" ".repeat(spaces_between_words));
            num_spaces -= spaces_between_words;
        }
        word_index += 1;
      }
      result.push(line);
      start_word = end_word;
    }
    result
    
  }
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
    assert_eq!(Solution_GaDo::format_string(words, max_width), expected);
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
    assert_eq!(Solution_GaDo::format_string(words, max_width), expected);
  }
}
