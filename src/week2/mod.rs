struct Solution {}

impl Solution {
    fn center_string(
        words: &Vec<String>,
        total_word: i32,
        total_length: i32,
        max_width: i32,
    ) -> String {
        let mut res = "".to_string();
        let space_len = max_width - total_length;
        if total_word == 0 {
            res.push_str(&words[0]);
            res.push_str(&String::from_utf8(vec![b' '; space_len as usize]).expect("Init failed"));
            return res;
        }
        let each_space_len = space_len / total_word;
        let space_word = String::from_utf8(vec![b' '; each_space_len as usize]).expect("Init failed");
        let mut remain_space = space_len % total_word;
        for (ind, word) in words.iter().enumerate() {
            res.push_str(word);
            if ind as i32 == total_word {
                break;
            }
            res.push_str(&space_word);
            if remain_space > 0 {
                res.push(' ');
                remain_space -= 1;
            }
        }
        res
    }

    fn format_string(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut acc_len: i32 = 0;
        let mut acc_word: i32 = -1;
        let mut acc = vec![];
        let mut res = vec![];
        for word in words.iter() {
            // If accumulated plus current work greater than max_width, use the accumulated
            if acc_len + word.len() as i32 + acc_word + 1 > max_width {
                res.push(Solution::center_string(&acc, acc_word, acc_len, max_width));
                acc_word = 0;
                acc_len = word.len() as i32;
                acc.clear();
                acc.push(word.clone());
            } else {
                acc_word += 1;
                acc_len += word.len() as i32;
                acc.push(word.clone());
            }
        }
        if acc_len > 0 {
            res.push(Solution::center_string(&acc, acc_word, acc_len, max_width));
        }
        res
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
        let expected = [
            "This    is   a",
            "simple    test",
            "case       for",
            "center        ",
            "justification.",
        ];
        assert_eq!(Solution::format_string(words, max_width), expected);
    }
}
