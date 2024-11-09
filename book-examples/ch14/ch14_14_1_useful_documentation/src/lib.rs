//! # ch14_ch14_14_1_useful_documentation
 //!
 //! `ch14_ch14_14_1_useful_documentation` is a collection of utilities to make performing
 //! certain calculations more convenient. 

/// Adds left value and right value and produces the result.
 ///
 /// # Examples
 ///
 /// ```
 /// let left = 5;
 /// let right = 10;
 /// let answer = ch14_14_1_useful_documentation::add(left, right);
 ///
 /// assert_eq!(15, answer);
 /// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
