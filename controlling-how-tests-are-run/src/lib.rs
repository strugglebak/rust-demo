#[cfg(test)]
mod tests {
  // printed values for passing tests
  // cargo test -- --show-output
  use super::*;

  #[test]
  fn this_test_will_pass() {
    let value = prints_and_returns_10(4);
    assert_eq!(10, value);
  }

  #[test]
  fn this_test_will_fail() {
    let value = prints_and_returns_10(8);
    assert_eq!(5, value);
  }
}

fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {}", a);
  10
}
