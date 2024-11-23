fn main() {
    // let config_max = Some(5);
    
    // match config_max {
    // Some(max) => println!("The maximum is configured to be {max}"),
    // _ => (),
    // }

    // other way of implementation
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }  
}
