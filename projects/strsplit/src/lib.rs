//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

// why do impl block as <'a> to understand this, let's go back to the struct example
/*
struct Foo<T> {}

impl Foo<T> {}  // compiler will through error as it doesn't know what <T> is...

impl<T> Foo<T> {} // by doing this, it will make the impl block as a generic impl block over type <T> 
*/

// subtyping == lifetimes (term used in Rustnomicon)
// '_ - anonmymous lifetime
// '_ - tick underscore - means a pattern that matches anything (type inferences for references) or otherwise don't consider this lifetime for the purpose of guessing
// 'a - tick a - specific lifetime same as 

// str -> [char] type (it only stores the start address and total size)
// &str -> &[char] it is a pointer to a sequence of characters
// String -> Vec<char> heap allocated. can shrink and grow dynamically
//
// String -> &str (possible) -- (AsRef)
// &str -> String (very trivial as need to copy the contents to heap in order to store it as a String type) -- (Clone)

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// let x = StrSplit;
// for part in x {
// }
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // find the delimiter in the string slice
        // if let Some(ref mut remainder /* &mut &'a str */) = self.remainder /* Option<&'a str> */
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            // dereferening
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    StrSplit::new(s, format!("{}", c))
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    // println!("letters : {:?}", letters);
    // assert!(letters.into_iter().eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    // println!("letters : {:?}", letters);
    // assert!(letters.into_iter().eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
