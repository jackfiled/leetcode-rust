/**
 * [59] Spiral Matrix II
 */
pub struct Solution {}

// submission codes start here

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];
        let (mut x, mut y) = (0, 0);
        let mut pos = 0;

        for i in 1..=n.pow(2) {
            result[x as usize][y as usize] = i;

            let (next_x, next_y) = (x + DIRECTIONS[pos].0, y + DIRECTIONS[pos].1);

            if next_x < 0
                || next_x >= n
                || next_y < 0
                || next_y >= n
                || result[next_x as usize][next_y as usize] != 0
            {
                pos = (pos + 1) % 4;
                x = x + DIRECTIONS[pos].0;
                y = y + DIRECTIONS[pos].1;
            } else {
                x = next_x;
                y = next_y;
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
    fn test_59() {
        assert_eq!(vec![vec![1]], Solution::generate_matrix(1));
        assert_eq!(
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            Solution::generate_matrix(3)
        );
    }
}
