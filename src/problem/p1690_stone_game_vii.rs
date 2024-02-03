/**
 * [1690] Stone Game VII
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut sum = vec![0; n + 1];

        for (index, value) in stones.iter().enumerate() {
            sum[index + 1] = sum[index] + *value;
        }

        let mut dp = vec![vec![0;n];n];

        for i in (0..=n-2).rev() {
            for j in i+1..n {
                dp[i][j] = (sum[j + 1] - sum[i + 1] - dp[i + 1][j]).max(
                    sum[j] - sum[i] - dp[i][j - 1]
                );
            }
        }

        dp[0][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1690() {
        assert_eq!(Solution::stone_game_vii(vec![5,3,1,4,2]), 6);
    }
}
