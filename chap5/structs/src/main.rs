struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User { // Creating a new User instance without struct update
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };

    let user3 = User {
        email: String::from("more@example.com"),
        ..user1
    };
}





