#![allow(dead_code)]
pub struct Solution_Long {}

impl Solution_Long {
  pub fn strobogrammatic_numbers(low: i32, high: i32) -> i32 {
    let mut count = 0;

    for i in low.to_string().len()..=high.to_string().len() {
      let mut start = vec!['0'; i];

      Solution_Long::backtrack(&mut start, &mut count, low, high, 0, i - 1);
    }

    count
  }

  pub fn backtrack(num_str: &mut [char], count: &mut i32, low: i32, high: i32, left: usize, right: usize) {
    if left > right {
      let num = num_str.iter().collect::<String>().parse::<i32>().unwrap();
      if num >= low && num <= high {
        *count += 1;
      }
      return;
    }

    for (l, r) in [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')].iter() {
      if left == right && l != r {
        continue;
      }
      num_str[left] = *l;
      num_str[right] = *r;
      if num_str.len() > 1 && num_str[0] == '0' {
        continue;
      }

      Solution_Long::backtrack(num_str, count, low, high, left + 1, right - 1);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn one_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(50, 100), 3);
  }

  #[test]
  pub fn two_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(10, 100), 4);
  }

  #[test]
  pub fn three_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(10, 1000), 16);
  }

  #[test]
  pub fn four_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(200, 5000), 14);
  }

  #[test]
  pub fn five_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(100, 10000), 32);
  }

  #[test]
  #[should_panic]
  pub fn six_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(5, 10000), 32);
  }

  #[test]
  #[should_panic]
  pub fn seven_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(1000, 500), 32);
  }

  #[test]
  #[should_panic]
  pub fn eight_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(10, 10000), 32);
  }

  #[test]
  pub fn full_strobogrammatic_numbers() {
    assert_eq!(Solution_Long::strobogrammatic_numbers(10, 10000), 36);
  }
}
