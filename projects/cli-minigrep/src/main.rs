use std::env;
use std::process;

use cli_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    // let config = Config::new(&args);
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    
    // using iterators idea
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // println!("Searching for {}", config.query);
    // println!("file {}", config.file_path);

    if let Err(e) = cli_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
