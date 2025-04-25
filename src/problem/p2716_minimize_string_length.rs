/**
 * [2716] Minimize String Length
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let set: HashSet<u8> = HashSet::from_iter(s.bytes());

        set.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2716() {
        assert_eq!(3, Solution::minimized_string_length("aaabc".to_owned()));
        assert_eq!(3, Solution::minimized_string_length("cbbd".to_owned()));
        assert_eq!(2, Solution::minimized_string_length("dddaaa".to_owned()));
    }
}
