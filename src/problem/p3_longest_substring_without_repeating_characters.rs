/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest <span data-keyword="substring-nonempty">substring</span> without repeating characters.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.cn/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::max;
use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut window = HashSet::new();
        let mut left = 0;
        let mut result = 0;

        for i in 0..chars.len() {
            while window.contains(&chars[i]) {
                window.remove(&chars[left]);
                left += 1;
            }

            window.insert(chars[i]);
            result = max(result, window.len());
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("abcabcbb"))
        );
        assert_eq!(
            1,
            Solution::length_of_longest_substring(String::from("bbbbb"))
        );
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("pwwkew"))
        );
    }
}
