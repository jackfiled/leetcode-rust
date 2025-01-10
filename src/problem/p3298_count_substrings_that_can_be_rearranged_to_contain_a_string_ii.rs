/**
 * [3298] Count Substrings That Can Be Rearranged to Contain a String II
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let word1: Vec<char> = word1.chars().collect();
        let mut word_map = HashMap::with_capacity(26);
        for c in word2.chars() {
            let entry = word_map.entry(c).or_insert(0);
            *entry -= 1;
        }

        let mut result = 0;
        let mut count = word_map.iter().filter(|c| c.1 < &0).count();

        let mut right = 0;
        for left in 0..word1.len() {
            while right < word1.len() && count > 0 {
                let entry = word_map.entry(word1[right]).or_insert(0);
                *entry += 1;
                if *entry == 0 {
                    count -= 1;
                }
                right += 1;
            }

            if count == 0 {
                result += (word1.len() - right + 1) as i64;
            }
            let entry = word_map.entry(word1[left]).or_insert(0);
            *entry -= 1;
            if *entry == -1 {
                count += 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3298() {
        assert_eq!(
            1,
            Solution::valid_substring_count("bcca".to_owned(), "abc".to_owned())
        );
        assert_eq!(
            10,
            Solution::valid_substring_count("abcabc".to_owned(), "abc".to_owned())
        );
        assert_eq!(
            0,
            Solution::valid_substring_count("abcabc".to_owned(), "aaabc".to_owned())
        );
    }
}
