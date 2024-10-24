#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        username: String::from("someuser"),
        email: String::from("someuser@xyz.com"),
        sign_in_count: 2,
        active: true,
    };

    // copy the values of user1 to the user2 except for the different email ID for user2
    // this method uses Struct Update syntax
    // if the email and username doesn't inherit from user1, then copy trait will be done 
    let user2 = User {
        email: String::from("someuser456@xyz.com"),
        ..user1
    };

    // as email is partially moved to user2 and user1 is not the owner anymore
    // user1 can't be used after definition of user2
    // println!("User 1 : {:?}", user1);   // this one will trigger the error.

    println!("User 2 : {:?}", user2);
}
