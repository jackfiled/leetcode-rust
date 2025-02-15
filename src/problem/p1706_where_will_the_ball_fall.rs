/**
 * [1706] Where Will the Ball Fall
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

const LEFT: i32 = -1;
const RIGHT: i32 = 1;

struct Grid {
    m: usize,
    n: usize,
    grid: Vec<Vec<i32>>,
    memory: HashMap<usize, HashMap<usize, Option<usize>>>,
}

impl Grid {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let m = grid.len();
        let n = grid[0].len();

        let mut memory = HashMap::new();

        // 如果可能到达m行
        // 就已经掉出去了
        let row = memory.entry(m).or_insert(HashMap::new());
        for i in 0..n {
            row.insert(i, Some(i));
        }

        Self { m, n, grid, memory }
    }

    fn search(&mut self, x: usize, y: usize) -> Option<usize> {
        if let Some(result) = self
            .memory
            .get(&x)
            .into_iter()
            .filter_map(|x| x.get(&y))
            .next()
        {
            return result.as_ref().map(|x| *x);
        }

        let result = if self.grid[x][y] == LEFT {
            // 挡板朝向左边
            if y > 0 {
                if self.grid[x][y - 1] == RIGHT {
                    None
                } else {
                    self.search(x + 1, y - 1)
                }
            } else {
                None
            }
        } else {
            // 挡板朝向右边
            if y < self.n - 1 {
                if self.grid[x][y + 1] == LEFT {
                    None
                } else {
                    self.search(x + 1, y + 1)
                }
            } else {
                None
            }
        };

        let row = self.memory.entry(x).or_insert(HashMap::new());
        row.insert(y, result.clone());

        result
    }
}

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut grid = Grid::new(grid);

        let mut result = Vec::with_capacity(grid.n);

        for i in 0..grid.n {
            result.push(if let Some(pos) = grid.search(0, i) {
                pos as i32
            } else {
                -1
            });
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1706() {
        assert_eq!(
            vec![1, -1, -1, -1, -1],
            Solution::find_ball(vec![
                vec![1, 1, 1, -1, -1],
                vec![1, 1, 1, -1, -1],
                vec![-1, -1, -1, 1, 1],
                vec![1, 1, 1, 1, -1],
                vec![-1, -1, -1, -1, -1]
            ])
        );
    }
}
