/**
 * [1278] Palindrome Partitioning III
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let length = s.len();

        // cost[i][j]
        // 将 s[i..=j] 修改为回文串需要的修改次数
        let mut cost = vec![vec![0; length]; length];

        for span in 2..=length {
            for i in 0..=length - span {
                let j = i + span - 1;

                cost[i][j] = cost[i + 1][j - 1] + if s[i] == s[j] { 0 } else { 1 };
            }
        }

        let k = k as usize;
        // dp[i][j]
        // 将s[..i]分割为j个回文串需要的修改次数
        let mut dp = vec![vec![i32::MAX; k + 1]; length + 1];

        for i in 1..=length {
            for j in 1..=i.min(k) {
                if j == 1 {
                    dp[i][j] = cost[0][i - 1];
                } else {
                    // 这里k从j - 1开始枚举是因为前面需要进行j - 1次分割时
                    // 字符串的长度必须大于等于j - 1
                    for k in j - 1..i {
                        dp[i][j] = dp[i][j].min(dp[k][j - 1] + cost[k][i - 1]);
                    }
                }
            }
        }

        dp[length][k]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1278() {
        assert_eq!(1, Solution::palindrome_partition("abc".to_owned(), 2));
        assert_eq!(0, Solution::palindrome_partition("aabbc".to_owned(), 3));
        assert_eq!(0, Solution::palindrome_partition("leetcode".to_owned(), 8));
    }
}
