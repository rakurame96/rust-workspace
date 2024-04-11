#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// #[derive(Debug)]
// struct User_1 {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

fn main() {
    println!("Hello, world!");
    // let word = "hello, world!";
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someusername123@rust.com"),
        sign_in_count: 1,
    };

    // printing entire structure contents using the debug trait
    println!("user1: {:?}", user1);

    // printing the individual fields
    println!("active: {}", user1.active);
    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("sign_in_count: {}", user1.sign_in_count);

    // let user_12 = User_1 {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someusername123@rust.com"),
    //     sign_in_count: 1,
    // };

    // println!("user_12: {:?}", user_12);

}
