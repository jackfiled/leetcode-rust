/**
 * [28] Find the Index of the First Occurrence in a String
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle)  {
            Some(pos) => pos as i32,
            None => -1   
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
    }
}
