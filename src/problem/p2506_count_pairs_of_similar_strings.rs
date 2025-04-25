/**
 * [2506] Count Pairs Of Similar Strings
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut map = HashMap::new();

        for word in words.iter() {
            let mut bits = 0;

            for c in word.chars() {
                bits = bits | (1 << (c as u8 - 'a' as u8))
            }

            let entry = map.entry(bits).or_insert(0);
            *entry += 1;
        }

        let mut result = 0;

        for &v in map.values() {
            // C(n, 2)
            result += v * (v - 1) / 2;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2506() {
        assert_eq!(
            2,
            Solution::similar_pairs(vec_string!("aba", "aabb", "abcd", "bac", "aabc"))
        );
        assert_eq!(3, Solution::similar_pairs(vec_string!("aabb", "ab", "ba")));
        assert_eq!(0, Solution::similar_pairs(vec_string!("nba", "cba", "dba")));
    }
}
