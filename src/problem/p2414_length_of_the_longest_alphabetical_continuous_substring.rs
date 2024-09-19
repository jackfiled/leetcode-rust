/**
 * [2414] Length of the Longest Alphabetical Continuous Substring
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s: Vec<u32> = s.chars().map(|c| c.into()).collect();

        let mut result = 1;
        let mut length = 1;
        let mut last_char = s[0];

        for i in 1..s.len() {
            if last_char + 1 == s[i] {
                length += 1;
                result = result.max(length);
            } else {
                length = 1;
            }
            last_char = s[i];
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2414() {
        assert_eq!(2, Solution::longest_continuous_substring("abacaba".to_string()));
        assert_eq!(5, Solution::longest_continuous_substring("abcde".to_string()));
    }
}
