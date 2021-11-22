
// to add a new state, we would only need to add a new struct
// and implement the trait methods on that one struct

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  // We add a default implementation for the content method that returns an empty string slice
  // That means we don’t need to implement content on the Draft and PendingReview structs
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    ""
  }
}
struct PendingReview {}
struct Draft {}
struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}

impl State for PendingReview {
  // self: Box<Self>
  // This syntax means the method is only valid when called on a Box holding the type
  // This syntax takes ownership of Box<Self>
  // invalidating the old state so the state value of the Post can transform into a new state
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    // when we request a review on a post already in the PendingReview state,
    // it should stay in the PendingReview state
    Self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}
impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}
impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn content(&self) -> &str {
    // as_ref
    // we want a reference to the value inside the Option
    // rather than ownership of the value
    // state is an Option<Box<dyn State>>
    // when we call as_ref, an Option<&Box<dyn State>> is returned
    // If we didn’t call as_ref, we would get an error
    // because we can’t move state out of the borrowed &self of the function parameter.

    // content(self)
    // self as the post parameter
    self.state.as_ref().unwrap().content(self)
  }

  // To consume the old state
  // the request_review method needs to take ownership of the state value
  pub fn request_review(&mut self) {
    // take the Some value out of the state field and leave a None in its place
    // because Rust doesn’t let us have unpopulated fields in structs
    // This lets us move the state value out of Post rather than borrowing it
    if let Some(s) = self.state.take() {
      // Then we’ll set the post’s state value to the result of this operation
      self.state = Some(s.request_review())
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

// static dispatch
// which is when the compiler knows what method you’re calling at compile time
// dynamic dispatch
// which is when the compiler can’t tell at compile time which method you’re calling

// When we use trait objects, Rust must use dynamic dispatch
// The compiler doesn’t know all the types that might be used
// with the code that is using trait objects
// at runtime, Rust uses the pointers inside the trait object to know which method to call
