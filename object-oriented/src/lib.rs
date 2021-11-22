pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
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
