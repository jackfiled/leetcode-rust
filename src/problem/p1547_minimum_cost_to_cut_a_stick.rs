/**
 * [1547] Minimum Cost to Cut a Stick
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let m = cuts.len();

        let mut cuts = cuts;
        cuts.sort_unstable();
        cuts.insert(0, 0);
        cuts.push(n);

        let mut dp = vec![vec![0; m + 2]; m + 2];

        for i in (1..=m).rev() {
            for j in i..=m {
                dp[i][j] = if i == j { 0 } else { i32::MAX };

                for k in i..=j {
                    dp[i][j] = dp[i][j].min(dp[i][k - 1] + dp[k + 1][j]);
                }
                dp[i][j] += cuts[j + 1] - cuts[i - 1];
            }
        }

        dp[1][m]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1547() {
        assert_eq!(16, Solution::min_cost(7, vec![1, 3, 4, 5]));
        assert_eq!(22, Solution::min_cost(9, vec![5, 6, 1, 4, 2]));
    }
}
