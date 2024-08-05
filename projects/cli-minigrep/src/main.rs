use std::env;   use std::error::Error;
// to read the commandline arguments
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
        }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    

    println!("Searching for {}", config.query);
    println!("file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

#[allow(unused)]
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let fs_contents = fs::read_to_string(config.file_path)?;
    
    Ok(())
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }
