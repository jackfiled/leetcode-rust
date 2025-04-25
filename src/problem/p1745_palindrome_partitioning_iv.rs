/**
 * [1745] Palindrome Partitioning IV
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.as_bytes();
        let length = s.len();

        let mut palindrome = vec![vec![true; length]; length];

        for span in 2..=length {
            for i in 0..=length - span {
                let j = i + span - 1;
                palindrome[i][j] = palindrome[i + 1][j - 1] && (s[i] == s[j])
            }
        }

        for i in 0..length - 2 {
            // 所有字符串都必须是非空的
            for j in i + 2..length {
                if palindrome[0][i] && palindrome[i + 1][j - 1] && palindrome[j][length - 1] {
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
    fn test_1745() {
        assert!(Solution::check_partitioning("abcbdd".to_owned()));
        assert!(!Solution::check_partitioning("bcbddxy".to_owned()));
    }
}
