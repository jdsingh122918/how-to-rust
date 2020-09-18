struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let username1 = String::from("jdsingh");
    let email = String::from("info@fermatsolutions.dev");
    let user1 = build_user(email, username1);
    println!(
        "Hello, {}, {}, {}, {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
