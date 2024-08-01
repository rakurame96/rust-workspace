use std::env;   // to read the commandline arguments

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
