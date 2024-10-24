/**
 * [289] Game of Life
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();

        let directions = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ];

        for i in 0..m {
            for j in 0..n {
                let mut count = 0;

                for (delta_x, delta_y) in directions.iter() {
                    let x = i as i32 + *delta_x;
                    let y = j as i32 + *delta_y;

                    if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                        let (x, y) = (x as usize, y as usize);

                        // 本来就是活细胞或者刚死的活细胞
                        if board[x][y] == 1 || board[x][y] == -1 {
                            count += 1;
                        }
                    }
                }

                if board[i][j] == 1 {
                    if count < 2 || count > 3 {
                        // 死亡的活细胞
                        board[i][j] = -1;
                    }
                }

                if board[i][j] == 0 {
                    if count == 3 {
                        // 诞生的新细胞
                        board[i][j] = 2;
                    }
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 2 {
                    board[i][j] = 1;
                }

                if board[i][j] == -1 {
                    board[i][j] = 0;
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
    fn test_289() {}
}
