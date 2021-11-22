pub trait Draw {
  fn draw(&self);
}

// A trait is object safe
// 1. The return type isn’t Self
// 2. There are no generic type parameters

// a method of trait is not object safe
// pub trait Clone {
//     fn clone(&self) -> Self;
// }

pub struct Screen {
  // pub components: Vec<Box<dyn Clone>>,
  pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      // trait objects allow for multiple concrete types
      // to fill in for the trait object at runtime
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}
impl Draw for Button {
  fn draw(&self) {

  }
}
