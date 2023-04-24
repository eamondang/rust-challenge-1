#![allow(dead_code)]
pub mod discord_name;

struct Solution;

impl Solution {
  fn strobogrammatic_numbers(low: i32, high: i32) -> i32 {
    // ...
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
