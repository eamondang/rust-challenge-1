pub struct Solution_C {}

impl Solution_C {
  pub fn is_strobogrammatic(n: i32) -> bool {
    let n = n.to_string();
    let mut m = String::new();
    for c in n.chars().rev() {
      match c {
        '0' => m.push('0'),
        '1' => m.push('1'),
        '6' => m.push('9'),
        '8' => m.push('8'),
        '9' => m.push('6'),
        _ => return false,
      }
    }
    n == m
  }

  pub fn strobogrammatic_numbers(low: i32, high: i32) -> i32 {
    let mut count = 0;
    for i in low..=high {
      if Self::is_strobogrammatic(i) {
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
    assert_eq!(Solution_C::strobogrammatic_numbers(50, 100), 3);
  }

  #[test]
  fn two_strobogrammatic_numbers() {
    assert_eq!(Solution_C::strobogrammatic_numbers(10, 100), 4);
  }

  #[test]
  fn three_strobogrammatic_numbers() {
    assert_eq!(Solution_C::strobogrammatic_numbers(10, 1000), 16);
  }

  #[test]
  fn four_strobogrammatic_numbers() {
    assert_eq!(Solution_C::strobogrammatic_numbers(200, 5000), 14);
  }

  #[test]
  fn five_strobogrammatic_numbers() {
    assert_eq!(Solution_C::strobogrammatic_numbers(100, 10000), 32);
  }

  #[test]
  #[should_panic]
  fn six_strobogrammatic_numbers() {
    assert_eq!(Solution_C::strobogrammatic_numbers(5, 10000), 32);
  }

  #[test]
  #[should_panic]
  fn seven_strobogrammatic_numbers() {
    assert_eq!(Solution_C::strobogrammatic_numbers(1000, 500), 32);
  }

  #[test]
  #[should_panic]
  fn eight_strobogrammatic_numbers() {
    assert_eq!(Solution_C::strobogrammatic_numbers(10, 10000), 32);
  }

  #[test]
  fn full_strobogrammatic_numbers() {
    assert_eq!(Solution_C::strobogrammatic_numbers(10, 10000), 36);
  }
}
