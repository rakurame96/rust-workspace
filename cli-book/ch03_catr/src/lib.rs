use std::error::Error;
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader};

#[allow(unused)]
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        // println!("{}", filename);
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),            
        }
    }
    Ok(())
} 

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch03_catr")
        .version("0.1.0")
        .author("Rakuram <blog.rakuram@gmail.com>")
        .about("Rust cat Command")
        .help_template(
            "{name} {version} \n\
             {author} \n\
             {about} \n\n\
             USAGE: \n    {usage}\n\n\
             {all-args}",
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                // .required(true)          // reason why required is not used is because we are using default value
                .num_args(1..)
                .default_value("-")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("number all output lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // let text: Vec<String> = matches
    //     .get_many("files")
    //     .expect("filename is required")
    //     .cloned()
    //     .collect();

    // println!("{:#?}", text);

    // let number_lines = matches.get_flag("number_lines");

    // let number_nonblank_lines = matches.get_flag("number_nonblank_lines");

    // println!("text: {:#?}", text);
    // println!("number_lines: {:#?}", number_lines);
    // println!("number_nonblank_lines: {:#?}", number_nonblank_lines);

    Ok(Config {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}