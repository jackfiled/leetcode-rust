/**
 * [130] Surrounded Regions
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn dfs(board: &mut Vec<Vec<char>>, x: i32, y: i32, m: i32, n: i32) {
        if x < 0 || x >= m || y < 0 || y >= n {
            return;
        }

        let (pos_x, pos_y) = (x as usize, y as usize);

        if board[pos_x][pos_y] != 'O' {
            return;
        }

        board[pos_x][pos_y] = 'A';

        Self::dfs(board, x - 1, y, m, n);
        Self::dfs(board, x + 1, y, m, n);
        Self::dfs(board, x, y - 1, m, n);
        Self::dfs(board, x, y + 1, m, n);
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();
        let (m_length, n_length) = (m as i32, n as i32);

        for i in 0..m {
            if board[i][0] == 'O' {
                Self::dfs(board, i as i32, 0, m_length, n_length);
            }
            if board[i][n - 1] == 'O' {
                Self::dfs(board, i as i32, n_length - 1, m_length, n_length);
            }
        }

        for i in 0..n {
            if board[0][i] == 'O' {
                Self::dfs(board, 0, i as i32, m_length, n_length);
            }
            if board[m - 1][i] == 'O' {
                Self::dfs(board, m_length - 1, i as i32, m_length, n_length);
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
                if board[i][j] == 'A' {
                    board[i][j] = 'O';
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_130() {}
}
