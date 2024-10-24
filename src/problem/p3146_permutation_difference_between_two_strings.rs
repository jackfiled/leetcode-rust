/**
 * [3146] Permutation Difference between Two Strings
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut map = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            map.insert(c, i as i32);
        }

        let mut result = 0;

        for (i, c) in t.chars().enumerate() {
            let target = map.get(&c).unwrap();

            result += (i as i32 - *target).abs()
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3146() {
        assert_eq!(
            2,
            Solution::find_permutation_difference("abc".to_owned(), "bac".to_owned())
        );
    }
}
