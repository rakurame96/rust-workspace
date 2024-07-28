pub use std::collections::HashMap;
pub use std::boxed::Box;
pub use std::string::ToString;

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
    // pattern 1
    // pattern => template
    (null) => {
        $crate::Json::Null  
    };
    
    // pattern 2
    // ([ ... ]) => {
    //     Json::Array(...)
    // };
    ([ $( $element:tt),* ]) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };

    // pattern 3
    // ({ ... }) => {
    //     Json::Object(...)
    // };
    ({ $( $key:tt : $value:tt),* }) => {
        // Json::Object(Box::new(vec![
        //     $( (key.to_string(), json!($value)) ),*
        // ].into_iter().collect()))
        {
            let mut fields = $crate::macroBox::new(
                $crate::macros::HashMap::new());
            $( 
                fields.insert($crate::macros::ToString::to_string($key), 
                                json!($value)); )*
            $crate::Json::Object(fields);
        }
    };

    // pattern 4
    // (???) => {
    //     Json::Boolean(...)
    // };
    ( $other:tt) => {
        $crate::Json::from($other)      // handles boolean, numbers, string
    };
    // pattern 5
    // (???) => {
    //     Json::Number(...)
    // };

    // pattern 6
    // (???) => {
    //     Json::String(...)
    // };
}

#[macro_export]
macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<i32> for Json {
    fn from(l: i32) -> Json {
        Json::Number(l as f64)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

// impl From<&'a str> for Json {
//     fn from(s: &'a str) -> Json {
//         Json::String(s.to_string())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_null() {
        assert_eq!(json!(null), Json::Null);
    }
}
