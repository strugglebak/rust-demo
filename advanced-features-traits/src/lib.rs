// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

// You’ll use default type parameters in two main ways:
// 1. To extend a type without breaking existing code
// 2. To allow customization in specific cases most users won’t need
impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}
