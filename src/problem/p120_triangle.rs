/**
 * [120] Triangle
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![i32::MAX; n];
        dp[0] = 0;

        for i in 0..n {
            let mut next_dp = vec![i32::MAX; n];
            for j in 0..=i {
                if j == 0 {
                    next_dp[j] = triangle[i][j] + dp[j];
                } else {
                    next_dp[j] = triangle[i][j] + dp[j - 1].min(dp[j]);
                }
            }

            dp = next_dp;
        }

        *dp.iter().min().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_120() {
        assert_eq!(
            11,
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );
    }
}
