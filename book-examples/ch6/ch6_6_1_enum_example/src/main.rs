// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

//     println!("print values of home : {:?}", home);
//     println!("print values of loopback : {:?}", loopback);
    
//     println!("value of IpAddr: {:?}", home.kind);
//     println!("value of address: {:?}", home.address);
    
//     // dbg!(&home);
// }


// another way of implementing the above function
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
   }

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(&home);
    dbg!(&loopback);
}