/**
 * [999] Available Captures for Rook
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let n = board.len();
        let (x, y) = (0..n)
            .map(|x| {
                let y = board[x].iter().enumerate().find_map(
                    |(y, &c)| {
                        if c == 'R' {
                            Some(y)
                        } else {
                            None
                        }
                    },
                );

                y.and_then(|y| Some((x, y)))
            })
            .flatten()
            .next()
            .unwrap();

        let mut result = 0;

        let mut dx = x;
        while dx > 0 {
            dx -= 1;

            match board[dx][y] {
                'p' => {
                    result += 1;
                    break;
                }
                'B' => {
                    break;
                }
                _ => {}
            }
        }

        let mut dx = x;
        while dx < n - 1 {
            dx += 1;

            match board[dx][y] {
                'p' => {
                    result += 1;
                    break;
                }
                'B' => {
                    break;
                }
                _ => {}
            }
        }

        let mut dy = y;
        while dy > 0 {
            dy -= 1;

            match board[x][dy] {
                'p' => {
                    result += 1;
                    break;
                }
                'B' => {
                    break;
                }
                _ => {}
            }
        }

        let mut dy = y;
        while dy < n - 1 {
            dy += 1;

            match board[x][dy] {
                'p' => {
                    result += 1;
                    break;
                }
                'B' => {
                    break;
                }
                _ => {}
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {

    #[test]
    fn test_999() {}
}
