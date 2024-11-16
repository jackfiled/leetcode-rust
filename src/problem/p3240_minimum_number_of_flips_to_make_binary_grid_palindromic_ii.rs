/**
 * [3240] Minimum Number of Flips to Make Binary Grid Palindromic II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut result = 0;

        // For the kernel
        for i in 0..m / 2 {
            for j in 0..n / 2 {
                let mut one_count = 0;

                if grid[i][j] == 1 {
                    one_count += 1;
                }

                if grid[m - 1 - i][j] == 1 {
                    one_count += 1;
                }

                if grid[i][n - 1 - j] == 1 {
                    one_count += 1;
                }

                if grid[m - 1 - i][n - 1 - j] == 1 {
                    one_count += 1;
                }

                result += one_count.min(4 - one_count);
            }
        }

        let mut one_count = 0;
        let mut flip_count = 0;

        if m % 2 == 1 {
            for i in 0..n / 2 {
                if grid[m / 2][i] != grid[m / 2][n - 1 - i] {
                    flip_count += 1;
                } else {
                    if grid[m / 2][i] == 1 {
                        one_count += 2;
                    }
                }
            }
        }

        if n % 2 == 1 {
            for i in 0..m / 2 {
                if grid[i][n / 2] != grid[m - 1 - i][n / 2] {
                    flip_count += 1;
                } else {
                    if grid[i][n / 2] == 1 {
                        one_count += 2;
                    }
                }
            }
        }

        if one_count % 4 == 2 {
            if flip_count == 0 {
                result += 2;
            }
        }
        result += flip_count;

        if m % 2 == 1 && n % 2 == 1 {
            if grid[m / 2][n / 2] == 1 {
                result += 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3240() {
        assert_eq!(2, Solution::min_flips(vec![vec![0, 0, 1], vec![1, 1, 1]]));
        assert_eq!(
            2,
            Solution::min_flips(vec![vec![0, 1], vec![0, 1], vec![0, 0]])
        );
        assert_eq!(
            3,
            Solution::min_flips(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
        );
        assert_eq!(2, Solution::min_flips(vec![vec![1], vec![1]]));
    }
}
