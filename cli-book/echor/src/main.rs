use clap::{Arg, ArgAction, Command};

/*
// default will be "first value to be the path of the program itself"
// and any arguments passed to the program while calling the cargo run
// ex: cargo run a b c
Args {
    inner: ["E:\\Learning-to-code\\rust-workspace\\cli-book\\target\\debug\\echor.exe", "a", "b", "c"]
    }
*/
fn main() {
    // println!("{:?}", std::env::args());
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Rakuram <blog.rakuram@gmail.com>")
        .about("Rust Echo Command")
        .help_template(
            "{name} {version} \n\
             {author} \n\
             {about} \n\n\
             USAGE: \n    {usage}\n\n\
             {all-args}",
        )
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input Text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print the newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    println!("{:#?}", matches);
}
