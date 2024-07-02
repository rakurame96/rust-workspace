//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
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
            remainder: haystack,
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
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            /* once found, copy them to until_delimiter from the start of string till where the delimiter present */
            let until_delimiter = &self.remainder[..next_delim]; 
            self.delimiter = &self.remainder[(next_delim + self.delimiter.len())..];    // till end of the string slice
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            // &'a str     &'static str (static lifetime)
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}
