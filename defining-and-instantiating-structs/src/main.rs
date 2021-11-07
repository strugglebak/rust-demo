fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user2@example.com");
    println!("{}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);
    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    println!("{}, {}, {}, {}", user2.active, user2.username, user2.email, user2.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
