fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user2@example.com");
    println!("{}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
    println!("{}, {}, {}, {}", user3.active, user3.username, user3.email, user3.sign_in_count);

    let user2 = User {
        email: String::from("user3@example.com"),
        ..user1
    };
    println!("{}, {}, {}, {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    println!("user1.email is {}", user1.email);

    // but it will be error
    // let user4 = User {
    //     ..user1
    // };
    // println!("user1.email is {}", user1.email);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);

    struct AlwaysEqual;
    let _sub = AlwaysEqual;
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
