/**
 * [2684] Maximum Number of Moves in a Grid
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![vec![0; n]; m];

        for j in (0..n - 1).rev() {
            for i in 0..m {
                // [row - 1, col + 1]
                if i != 0 {
                    if grid[i - 1][j + 1] > grid[i][j] {
                        dp[i][j] = dp[i][j].max(dp[i - 1][j + 1] + 1);
                    }
                }

                // [row, col + 1]
                if grid[i][j + 1] > grid[i][j] {
                    dp[i][j] = dp[i][j].max(dp[i][j + 1] + 1);
                }

                // [row + 1, col + 1]
                if i != m - 1 {
                    if grid[i + 1][j + 1] > grid[i][j] {
                        dp[i][j] = dp[i][j].max(dp[i + 1][j + 1] + 1);
                    }
                }
            }
        }

        let mut result = 0;

        for i in 0..m {
            result = result.max(dp[i][0]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2684() {
        assert_eq!(
            3,
            Solution::max_moves(vec![
                vec![2, 4, 3, 5],
                vec![5, 4, 9, 3],
                vec![3, 4, 2, 11],
                vec![10, 9, 13, 15]
            ])
        );
        assert_eq!(
            0,
            Solution::max_moves(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]])
        );
    }
}
