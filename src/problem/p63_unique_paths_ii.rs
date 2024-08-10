/**
 * [63] Unique Paths II
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = if obstacle_grid[0][0] == 1 {
            0
        } else {
            1
        };

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    continue;
                }

                let mut value = 0;
                if i != 0 {
                    value += dp[i - 1][j];
                }

                if j != 0 {
                    value += dp[i][j - 1];
                }

                if i != 0 || j != 0 {
                    dp[i][j] = value;
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_63() {
        assert_eq!(2, Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]));
        assert_eq!(0, Solution::unique_paths_with_obstacles(vec![vec![1]]));
    }
}
