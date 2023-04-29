pub struct Solution_NhatQuan;

const PAIRS: [[char; 2]; 5] = [['0', '0'], ['1', '1'], ['6', '9'], ['8', '8'], ['9', '6']];

impl Solution_NhatQuan {
  pub fn strobogrammatic_numbers(low: i32, high: i32) -> i32 {
    let low_str = low.to_string();
    let high_str = high.to_string();
    let mut count = 0;

    for len in low_str.len()..=high_str.len() {
      let mut c = vec![' '; len];
      Solution_NhatQuan::recursion(&low_str, &high_str, &mut c, 0, len - 1, &mut count);
    }

    count
  }

  pub fn recursion(low: &str, high: &str, c: &mut [char], left: usize, right: usize, count: &mut i32) {
    if left > right {
      let s: String = c.iter().collect();
      if (s.len() == low.len() && s < low.to_string()) || (s.len() == high.len() && s > high.to_string()) {
        return;
      }
      *count += 1;
      return;
    }

    for p in &PAIRS {
      c[left] = p[0];
      c[right] = p[1];
      if c.len() != 1 && c[0] == '0' {
        continue;
      }
      if left == right && p[0] != p[1] {
        continue;
      }
      Solution_NhatQuan::recursion(low, high, c, left + 1, right - 1, count);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(50, 100), 3);
  }

  #[test]
  fn two_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(10, 100), 4);
  }

  #[test]
  fn three_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(10, 1000), 16);
  }

  #[test]
  fn four_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(200, 5000), 14);
  }

  #[test]
  fn five_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(100, 10000), 32);
  }

  #[test]
  #[should_panic]
  fn six_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(5, 10000), 32);
  }

  #[test]
  #[should_panic]
  fn seven_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(1000, 500), 32);
  }

  #[test]
  #[should_panic]
  fn eight_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(10, 10000), 32);
  }

  #[test]
  fn full_strobogrammatic_numbers() {
    assert_eq!(Solution_NhatQuan::strobogrammatic_numbers(10, 10000), 36);
  }
}
