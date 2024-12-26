/**
 * [3083] Existence of a Substring in a String and Its Reverse
 */
pub struct Solution {}

// submission codes start here
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();

        let mut map = HashMap::new();

        for i in (1..n).rev() {
            let entry = map.entry(s[i]).or_insert(HashSet::new());
            entry.insert(s[i - 1]);
        }

        for i in 0..n - 1 {
            if let Some(set) = map.get(&s[i]) {
                if set.contains(&s[i + 1]) {
                    return true;
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3083() {
        assert!(Solution::is_substring_present("leetcode".to_owned()));
        assert!(Solution::is_substring_present("abcba".to_owned()));
        assert!(!Solution::is_substring_present("abcd".to_owned()));
    }
}
