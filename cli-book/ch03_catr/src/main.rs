fn main() {
    if let Err(e) = ch03_catr::get_args().and_then(ch03_catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
