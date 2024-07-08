/**
 * [79] Word Search
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
       
        let (m, n) = (board.len(), board[0].len());

        let mut visited = vec![vec![false;n];m];
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == word[0] {
                    if Self::search(&board, &word, &mut visited, i as i32, j as i32, 0) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn search(board: &Vec<Vec<char>>, word: &Vec<char>, visited: &mut Vec<Vec<bool>>, x: i32, y: i32, pos: usize) -> bool {
        if pos == word.len() {
            return true;
        }

        let (m, n) = (board.len() as i32, board[0].len() as i32);

        if x < 0 || x >= m || y < 0 || y >= n {
            return false;
        }
        
        let (x_pos, y_pos) = (x as usize, y as usize);
        
        if visited[x_pos][y_pos] {
            return false;
        }

        visited[x_pos][y_pos] = true;
        if board[x_pos][y_pos] == word[pos] {
            if Self::search(board, word, visited, x + 1, y, pos + 1) {
                return true;
            }

            if Self::search(board, word, visited, x - 1, y, pos + 1) {
                return true;
            }

            if Self::search(board, word, visited, x, y + 1, pos + 1) {
                return true;
            }

            if Self::search(board, word, visited, x, y - 1, pos + 1) {
                return true;
            }
        }
        visited[x_pos][y_pos] = false;

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_79() {}
}
