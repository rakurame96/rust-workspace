use std::env;   // to read the commandline arguments
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    // let query = &args[1];
    // let file_path = &args[2];

    let (query, file_path) = parse_config(&args);

    

    println!("Searching for {}", query);
    println!("file {}", file_path);

    let fs_contents = fs::read_to_string(file_path)
                    .expect("should have been able to read the file");
    
    println!("With text: \n{}", fs_contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
