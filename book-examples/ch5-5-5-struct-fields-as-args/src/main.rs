#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// by default, rust structs also immutable
fn main() {

    let username = String::from("someuser456");
    let email = String::from("someuser456@xyz.com");
    
    let user = build_user(username, email);

    println!("user : {:?}", user);
}

fn build_user(username: String, email: String) -> User {
    User {
        // username: username   // also valid statement
        username,               // shorthand method
        email, 
        active: true, 
        sign_in_count: 3,
    }
}