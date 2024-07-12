pub fn greeting(name: &str) -> String {
    //format!("Hello {name}")
    String::from("hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("raku");
        // without custom messages
        assert!(result.contains("rakuram"));

        //with custom message in the assert! macro
        assert!(
            result.contains("rakuram"),
            "Greeting did not contain name, value was {}", result
        );
    }
}
