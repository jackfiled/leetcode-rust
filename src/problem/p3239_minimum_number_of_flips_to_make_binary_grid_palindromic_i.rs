/**
 * [3239] Minimum Number of Flips to Make Binary Grid Palindromic I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut row = 0;

        for i in 0..m {
            for j in 0..n / 2 {
                if grid[i][j] != grid[i][n - j - 1] {
                    row += 1;
                }
            }
        }

        let mut column = 0;

        for i in 0..n {
            for j in 0..m / 2 {
                if grid[j][i] != grid[m - j - 1][i] {
                    column += 1;
                }
            }
        }

        row.min(column)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3239() {
        assert_eq!(
            2,
            Solution::min_flips(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]])
        );
        assert_eq!(
            1,
            Solution::min_flips(vec![vec![0, 1], vec![0, 1], vec![0, 0]])
        );
    }
}
