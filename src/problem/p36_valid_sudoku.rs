/**
 * [36] Valid Sudoku
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::new();

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }

                if set.contains(&board[i][j]) {
                    return false;
                }

                set.insert(board[i][j]);
            }

            set.clear();
        }

        for i in 0..9 {
            for j in 0..9 {
                if board[j][i] == '.' {
                    continue;
                }

                if set.contains(&board[j][i]) {
                    return false;
                }

                set.insert(board[j][i]);
            }

            set.clear();
        }

        for w in (0..9).step_by(3) {
            for h in (0..9).step_by(3) {
                for i in 0..3 {
                    for j in 0..3 {
                        if board[w + i][h + j] == '.' {
                            continue;
                        }

                        if set.contains(&board[w + i][h + j]) {
                            return false;
                        }

                        set.insert(board[w + i][h + j]);
                    }
                }

                set.clear();
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_36() {}
}
