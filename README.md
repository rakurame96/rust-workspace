#Below command will create the GIT repo with initialization

cargo new <package_name>

#Below command will create the GIT repo without initialization

cargo new <package_name> --vcs none

#to build

cargo build

#to run

cargo run

#for debug information

rustc --explain <ErrorCode>
Example: rustc --explain E0277

#to compile single file

rustc <rust ssource file name>
Example: rustc main.rs

Reference material:
1. [Why Rust](https://inpyjama.com/day0-why-rust/)
2. [Day 1: Setting Up the Environment](https://inpyjama.com/day1-setting-up-the-environment/)