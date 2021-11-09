use rand::Rng;

use std::{cmp::Ordering, io};

// be careful using it !!!
use std::collections::*;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..101);
  println!("The value is {}", secret_number);
}
