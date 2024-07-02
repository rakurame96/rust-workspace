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
        if let Some(ref mut remainder /* &mut &'a str */) = self.remainder /* Option<&'a str> */
        {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                // dereferening
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
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
