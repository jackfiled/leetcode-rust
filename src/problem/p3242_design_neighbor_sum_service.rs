/**
 * [3242] Design Neighbor Sum Service
 */
pub struct Solution {}

// submission codes start here

struct NeighborSum {
    adjacent_values: Vec<i32>,
    diagonal_values: Vec<i32>,
}

const DIAGONAL_DELTA: [(i32, i32); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];
const ADJACENT_DELTA: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len();
        let mut adjacent_values = vec![0; n * n];
        let mut diagonal_values = vec![0; n * n];
        let n = n as i32;

        for i in 0..n {
            for j in 0..n {
                let mut adjacent_value = 0;
                let mut diagonal_value = 0;
                for (&(x_1, y_1), &(x_2, y_2)) in ADJACENT_DELTA.iter().zip(DIAGONAL_DELTA.iter()) {
                    let (x, y) = (i + x_1, j + y_1);

                    if x >= 0 && x < n && y >= 0 && y < n {
                        adjacent_value += grid[x as usize][y as usize];
                    }

                    let (x, y) = (i + x_2, j + y_2);

                    if x >= 0 && x < n && y >= 0 && y < n {
                        diagonal_value += grid[x as usize][y as usize];
                    }
                }
                adjacent_values[grid[i as usize][j as usize] as usize] = adjacent_value;
                diagonal_values[grid[i as usize][j as usize] as usize] = diagonal_value;
            }
        }

        Self {
            adjacent_values,
            diagonal_values,
        }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.adjacent_values[value as usize]
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.diagonal_values[value as usize]
    }
}

/**
 * Your NeighborSum object will be instantiated and called as such:
 * let obj = NeighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3242() {
        let sum = NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);

        assert_eq!(6, sum.adjacent_sum(1));
        assert_eq!(16, sum.adjacent_sum(4));
        assert_eq!(16, sum.diagonal_sum(4));
        assert_eq!(4, sum.diagonal_sum(8));
    }
}
