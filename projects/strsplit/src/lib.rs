//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
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

// str -> [char] type (collecton or sequence of characters. it only stores the start address and total size)
// &str -> &[char] it is a pointer to a sequence of characters (FAT Pointer) (Knows starting address and how long the string is - length)
// String -> Vec<char> heap allocated. can shrink and grow dynamically. From String, I can retrieve the refernce to &str.
//
// String -> &str (possible) -- (AsRef)
// &str -> String (very trivial as need to copy the contents to heap in order to store it as a String type) -- (Clone)

// The term "fat pointer" is used to refer to references and raw pointers to dynamically sized types (DSTs) – slices or trait objects. 
// A fat pointer contains a pointer plus some information that makes the DST "complete" (e.g. the length).

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// let x = StrSplit;
// for part in x {
// }
impl<'haystack, D> Iterator for StrSplit<'haystack, D> 
where
    D: Delimiter
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // find the delimiter in the string slice
        // if let Some(ref mut remainder /* &mut &'a str */) = self.remainder /* Option<&'a str> */
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            // dereferening
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn until_char<'delimiter>(s: &'delimiter str, c: char) -> &'delimiter str {
    // let delim = format!("{}", c);
    StrSplit::new(s, c)
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
