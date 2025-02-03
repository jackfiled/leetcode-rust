/**
 * [680] Valid Palindrome II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let length = s.len();

        for i in 0..length / 2 {
            if s[i] != s[s.len() - 1 - i] {
                // 这B玩意儿还带回溯的
                // 需要选择从左边删还是从右边删
                // 这里直接强制一下
                return Self::check_palindrome(&s[i + 1..s.len() - i])
                    || Self::check_palindrome(&s[i..s.len() - 1 - i]);
            }
        }

        true
    }

    fn check_palindrome(s: &[char]) -> bool {
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
                return false;
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_680() {
        assert!(Solution::valid_palindrome(
            "cupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupucu".to_owned()
        ));
        assert!(Solution::valid_palindrome("cbbcc".to_owned()));
        assert!(Solution::valid_palindrome("aba".to_owned()));
        assert!(Solution::valid_palindrome("abca".to_owned()));
        assert!(!Solution::valid_palindrome("abc".to_owned()));
    }
}
