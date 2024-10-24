/**
 * [200] Number of Islands
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn is_in_grid(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        x >= 0 && x < m && y >= 0 && y < n
    }

    fn dfs(grid: &mut Vec<Vec<char>>, x: i32, y: i32) {
        if !Self::is_in_grid(x, y, grid) {
            return;
        }

        if grid[x as usize][y as usize] != '1' {
            return;
        }
        grid[x as usize][y as usize] = '2';

        Self::dfs(grid, x - 1, y);
        Self::dfs(grid, x + 1, y);
        Self::dfs(grid, x, y + 1);
        Self::dfs(grid, x, y - 1);
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = 0;
        let mut grid = grid;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    result += 1;
                    Self::dfs(&mut grid, i as i32, j as i32);
                }
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
    fn test_200() {}
}
