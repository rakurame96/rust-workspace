use std::collections::HashMap;

/*
Our example JSON data
let students = json!([
    {
        "name": "Jim Blandy",
        "class_of": 1926,
        "major": "Tibetan throat singing"
    },
    {
        "name": "Jason Orendorff",
        "class_of": 1702,
        "major": "Knots"
    }
]);
*/

#[derive(Clone, PartialEq, Debug)]
#[allow(unused)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>), 
    Object(Box<HashMap<String, Json>>)
}

#[macro_export]
macro_rules! json {
    (null) => {
        Json::Null  
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_null() {
        assert_eq!(json!(null), Json::Null);
    }
}
