#![allow(dead_code)]
pub struct Solution_Cuong {}

impl Solution_Cuong {
  pub fn strobogrammatic_numbers(low: i32, high: i32) -> i32 {
    let mut count = 0;
    let mut num_digits = low.to_string().len();
    while num_digits <= high.to_string().len() {
      count += Solution_Cuong::count_strobogrammatic_numbers(num_digits, low, high);
      num_digits += 1;
    }
    count
  }

  pub fn count_strobogrammatic_numbers(num_digits: usize, low: i32, high: i32) -> i32 {
    let candidates: [char; 5] = ['0', '1', '6', '8', '9'];
    let mut count = 0;
    let mut num_str = vec!['0'; num_digits];
    Solution_Cuong::count_helper(&mut num_str, &candidates, low, high, &mut count, 0, num_digits - 1);
    count
  }

  pub fn count_helper(
    num_str: &mut Vec<char>,
    candidates: &[char; 5],
    low: i32,
    high: i32,
    count: &mut i32,
    left: usize,
    right: usize,
  ) {
    if left > right {
      let num = num_str.iter().collect::<String>().parse::<i32>().unwrap();
      if num >= low && num <= high {
        *count += 1;
      }
      return;
    }

    for i in 0..candidates.len() {
      num_str[left] = candidates[i];
      num_str[right] = match candidates[i] {
        '0' => '0',
        '1' => '1',
        '6' => '9',
        '8' => '8',
        '9' => '6',
        _ => continue,
      };
      if (num_str.len() > 1 && num_str[0] == '0') || (left == right && candidates[i] != num_str[left]) {
        continue;
      }
      Solution_Cuong::count_helper(num_str, candidates, low, high, count, left + 1, right - 1);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn one_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(50, 100), 3);
  }

  #[test]
  pub fn two_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(10, 100), 4);
  }

  #[test]
  pub fn three_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(10, 1000), 16);
  }

  #[test]
  pub fn four_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(200, 5000), 14);
  }

  #[test]
  pub fn five_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(100, 10000), 32);
  }

  #[test]
  #[should_panic]
  pub fn six_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(5, 10000), 32);
  }

  #[test]
  #[should_panic]
  pub fn seven_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(1000, 500), 32);
  }

  #[test]
  #[should_panic]
  pub fn eight_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(10, 10000), 32);
  }

  #[test]
  pub fn full_strobogrammatic_numbers() {
    assert_eq!(Solution_Cuong::strobogrammatic_numbers(10, 10000), 36);
  }
}
