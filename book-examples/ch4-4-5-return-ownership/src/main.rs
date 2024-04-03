fn main() {
    let s1 = String::from("hello");
    
    let (s2, len) = calculate_length(s1);

    println!("String : {}, String_Length : {}", s2, len);
}

fn calculate_length(mut s: String) -> (String, usize) {
    let length = s.len();
    
    println!("String Length : {length}");
    s.push_str(" world!");
    
    let length = s.len();            // len() returns the length of a String
    println!("String Length : {length}");
 
    (s, length)
}