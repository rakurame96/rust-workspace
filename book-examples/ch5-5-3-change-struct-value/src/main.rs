#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// by default, rust structs also immutable
fn main() {
    let mut user1 = User {
        username: String::from("someuser"),
        email: String::from("someuser@xyz.com"),
        sign_in_count: 2,
        active: true,
    };
    println!("User1 before modification : {:?}", user1);

    user1.username = String::from("someuser123");

    println!("User1 after modification : {:?}", user1);
}
