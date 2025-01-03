#[derive(Debug)]
#[allow(unused)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 5, height: 12},
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
