/**
 * [2697] Lexicographically Smallest Palindrome
 *
 * You are given a string <code node="[object Object]">s consisting of lowercase English letters, and you are allowed to perform operations on it. In one operation, you can replace a character in <code node="[object Object]">s with another lowercase English letter.
 * Your task is to make <code node="[object Object]">s a palindrome with the minimum number of operations possible. If there are multiple palindromes that can be <meta charset="utf-8" />made using the minimum number of operations, <meta charset="utf-8" />make the lexicographically smallest one.
 * A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ, string a has a letter that appears earlier in the alphabet than the corresponding letter in b.
 * Return the resulting palindrome string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "egcfe"
 * Output: "efcfe"
 * Explanation: The minimum number of operations to make "egcfe" a palindrome is 1, and the lexicographically smallest palindrome string we can get by modifying one character is "efcfe", by changing 'g'.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abcd"
 * Output: "abba"
 * Explanation: The minimum number of operations to make "abcd" a palindrome is 2, and the lexicographically smallest palindrome string we can get by modifying two characters is "abba".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "seven"
 * Output: "neven"
 * Explanation: The minimum number of operations to make "seven" a palindrome is 1, and the lexicographically smallest palindrome string we can get by modifying one character is "neven".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/lexicographically-smallest-palindrome/
// discuss: https://leetcode.cn/problems/lexicographically-smallest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::min;
impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut result: Vec<u8> = s.bytes().collect();
        let mut left = 0;
        let mut right = result.len() - 1;

        while left < right {
            if result[left] != result[right] {
                let value = min(result[left], result[right]);

                result[left] = value;
                result[right] = value;
            }

            left = left + 1;
            right = right - 1;
        }

        String::from_utf8(result).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2697() {
        assert_eq!(
            String::from("efcfe"),
            Solution::make_smallest_palindrome(String::from("egcfe"))
        );
        assert_eq!(
            String::from("abba"),
            Solution::make_smallest_palindrome(String::from("abcd"))
        );
        assert_eq!(
            String::from("neven"),
            Solution::make_smallest_palindrome(String::from("seven"))
        );
    }
}
