/**
 * [125] Valid Palindrome
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.to_lowercase();
        let mut result = Vec::new();

        for c in s.chars() {
            if c.is_ascii_alphabetic() || c.is_digit(10) {
                result.push(c);
            }
        }

        if result.len() <= 1 {
            return true;
        }

        let mut i = 0;
        let mut j = result.len() - 1;

        while i < j {
            if result[i] != result[j] {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_owned()
        ));
        assert!(!Solution::is_palindrome("0P".to_owned()));
    }
}
