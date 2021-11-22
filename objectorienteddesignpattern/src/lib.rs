
trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
}
struct PendingReview {}
struct Draft {}
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
}
impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
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
    ""
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
}
