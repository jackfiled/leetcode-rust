/**
 * [688] Knight Probability in Chessboard
 */
pub struct Solution {}

// submission codes start here

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -2),
    (1, -2),
    (-1, 2),
    (1, 2),
    (-2, -1),
    (2, -1),
    (-2, 1),
    (2, 1),
];

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let (n, k) = (n as usize, k as usize);
        let (row, column) = (row as usize, column as usize);

        let mut dp = vec![vec![vec![0f64; n]; n]; k + 1];

        for step in 0..=k {
            for x in 0..n {
                for y in 0..n {
                    if step == 0 {
                        dp[step][x][y] = 1f64;
                    } else {
                        for &(dx, dy) in DIRECTIONS.iter() {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                                let (nx, ny) = (nx as usize, ny as usize);

                                dp[step][x][y] += dp[step - 1][nx][ny] / 8f64;
                            }
                        }
                    }
                }
            }
        }

        dp[k][row][column]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_688() {
        assert_eq!(0.0625, Solution::knight_probability(3, 2, 0, 0));
        assert_eq!(1.0, Solution::knight_probability(1, 0, 0, 0));
    }
}
