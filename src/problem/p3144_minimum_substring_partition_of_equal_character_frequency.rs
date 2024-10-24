/**
 * [3144] Minimum Substring Partition of Equal Character Frequency
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();

        let mut dp = vec![i32::MAX; n + 1];
        let mut char_count = HashMap::new();
        dp[0] = 0;

        for i in 1..=n {
            let mut max_count = 0;
            char_count.clear();

            for j in (1..=i).rev() {
                let entry = char_count.entry(s[j - 1]).or_insert(0);
                *entry += 1;
                max_count = max_count.max(*entry);

                if max_count * char_count.len() == i - j + 1 && dp[j - 1] != i32::MAX {
                    dp[i] = dp[i].min(dp[j - 1] + 1);
                }
            }
        }

        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3144() {
        assert_eq!(
            3,
            Solution::minimum_substrings_in_partition("fabccddg".to_owned())
        );
        assert_eq!(
            2,
            Solution::minimum_substrings_in_partition("abababaccddb".to_owned())
        );
    }
}
