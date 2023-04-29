#![allow(dead_code)]
pub struct Solution_BaHoang;

impl Solution_BaHoang {
  const rev: [i32; 5] = [0, 1, 9, 8, 6];
  pub fn strobogrammatic_numbers_with_length(low: i32, high: i32, len: i32, cur: i32, pos: i32) -> i32 {
    if pos >= (len + 1) / 2 {
      return if cur > low && cur < high { 1 } else { 0 };
    }
    let mut res = 0;
    for (index, digit) in [0, 1, 6, 8, 9].iter().enumerate() {
      // if pos = 0, the leading number must not be zero
      if pos == 0 && *digit == 0 {
        continue;
      }
      // if pos is the center of the number, the number must not be 6 or 9
      if (pos == len / 2) && (*digit == 6 || *digit == 9) {
        continue;
      }
      let mut new_cur = cur + digit * i32::pow(10, pos as u32);
      // if pos the not the center, add the number in other part
      if pos < len / 2 {
        new_cur += Solution_BaHoang::rev[index] * i32::pow(10, (len - pos - 1) as u32);
      }
      res += Solution_BaHoang::strobogrammatic_numbers_with_length(low, high, len, new_cur, pos + 1);
    }
    res
  }
  pub fn strobogrammatic_numbers(low: i32, high: i32) -> i32 {
    let mut res = 0;
    for i in 1..5 {
      if i32::pow(10, i as u32) - 1 <= low {
        continue;
      }
      if i32::pow(10, (i - 1) as u32) >= high {
        break;
      }

      res += Solution_BaHoang::strobogrammatic_numbers_with_length(low, high, i, 0, 0);
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn one_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(50, 100), 3);
  }

  #[test]
  pub fn two_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(10, 100), 4);
  }

  #[test]
  pub fn three_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(10, 1000), 16);
  }

  #[test]
  pub fn four_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(200, 5000), 14);
  }

  #[test]
  pub fn five_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(100, 10000), 32);
  }

  #[test]
  #[should_panic]
  pub fn six_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(5, 10000), 32);
  }

  #[test]
  #[should_panic]
  pub fn seven_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(1000, 500), 32);
  }

  #[test]
  #[should_panic]
  pub fn eight_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(10, 10000), 32);
  }

  #[test]
  pub fn full_strobogrammatic_numbers() {
    assert_eq!(Solution_BaHoang::strobogrammatic_numbers(10, 10000), 36);
  }
}
