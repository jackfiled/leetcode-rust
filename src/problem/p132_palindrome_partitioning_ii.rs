/**
 * [132] Palindrome Partitioning II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let length = s.len();

        let mut palindorme = vec![vec![true; length]; length];

        for i in (0..length).rev() {
            for j in i + 1..length {
                palindorme[i][j] = (s[i] == s[j]) && palindorme[i + 1][j - 1]
            }
        }

        let mut dp = vec![i32::MAX; length];

        for i in 0..length {
            if palindorme[0][i] {
                dp[i] = 0;
            } else {
                for j in 0..i {
                    if palindorme[j + 1][i] {
                        dp[i] = dp[i].min(dp[j] + 1);
                    }
                }
            }
        }

        dp[length - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_132() {
        assert_eq!(1, Solution::min_cut("aab".to_owned()));
        assert_eq!(0, Solution::min_cut("a".to_owned()));
        assert_eq!(1, Solution::min_cut("ab".to_owned()));
    }
}
