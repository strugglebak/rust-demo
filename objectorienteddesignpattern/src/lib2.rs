// invalid states are now impossible
// because of the type system and the type checking
// that happens at compile time!

// Object-oriented patterns won’t always be the best solution in Rust due to certain features,
// like ownership, that object-oriented languages don’t have.
pub struct Post {
  content: String,
}

pub struct DraftPost {
  content: String,
}

pub struct PendingReviewPost {
  content: String,
}
impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }
  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}

// fn main() {
//     let mut post = Post::new();

//     post.add_text("I ate a salad for lunch today");

//     let post = post.request_review();

//     let post = post.approve();

//     assert_eq!("I ate a salad for lunch today", post.content());
// }
