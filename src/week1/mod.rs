#![allow(dead_code)]
pub mod discord_name;

pub struct Solution;

impl Solution {
  pub fn is_strobogrammatic(number: &i32) -> bool {
    let strobogrammatic_pairs: Vec<(i32, i32)> = vec![(0, 0), (1, 1), (6, 9), (8, 8), (9, 6)];
    let digits: Vec<i32> = number
      .to_string()
      .chars()
      .map(|c| c.to_digit(10).unwrap() as i32)
      .collect();
    let mut left = 0;
    let mut right = digits.len() - 1;
    while left <= right {
      let pair = (digits[left], digits[right]);
      if !strobogrammatic_pairs.contains(&pair) {
        return false;
      }
      left += 1;
      right -= 1;
    }
    true
  }

  pub fn strobogrammatic_numbers(low: i32, high: i32) -> i32 {
    let mut count = 0;
    for num in low..=high {
      if Solution::is_strobogrammatic(&num) {
        count += 1;
      }
    }
    count
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(50, 100), 3);
  }

  #[test]
  fn two_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(10, 100), 4);
  }

  #[test]
  fn three_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(10, 1000), 16);
  }

  #[test]
  fn four_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(200, 5000), 14);
  }

  #[test]
  fn five_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(100, 10000), 32);
  }

  #[test]
  #[should_panic]
  fn six_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(5, 10000), 32);
  }

  #[test]
  #[should_panic]
  fn seven_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(1000, 500), 32);
  }

  #[test]
  #[should_panic]
  fn eight_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(10, 10000), 32);
  }

  #[test]
  fn full_strobogrammatic_numbers() {
    assert_eq!(Solution::strobogrammatic_numbers(10, 10000), 36);
  }
}
