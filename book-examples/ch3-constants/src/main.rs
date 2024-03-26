// consts GLOBAL_CONST = 180;     // compile error as constants always needs to be annotated with the datatype
const GLOBAL_CONST: u32 = 180;

fn main() {
    println!("Value of global constant variable = {}", GLOBAL_CONST);

    const LOCAL_CONST: f32 = 0.001;

    println!("Value of local constant variable = {}", LOCAL_CONST);
}
