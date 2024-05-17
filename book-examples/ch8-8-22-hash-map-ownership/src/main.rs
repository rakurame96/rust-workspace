use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try
    // using them and see what compiler error you get!

    let field_name = String::from("Favourite number");
    let field_value = 6;

    let mut map_1 = HashMap::new();
    map_1.insert(field_name, field_value);
    // but accessing field_name is not allowed. as ownership transferred to the map_1
    println!("field value : {}", field_value);  // accessing field_value at this point is not a problem incase the datatype supports copy trait


    println!("map : {:?}", map);
    println!("map_1 : {:?}", map_1);
}
