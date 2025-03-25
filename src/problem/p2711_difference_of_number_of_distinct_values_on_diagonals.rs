/**
 * [2711] Difference of Number of Distinct Values on Diagonals
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = vec![vec![0; n]; m];

        Self::iterate_diagonal(&grid, &mut result, (0..m).rev().map(|i| (i, 0)));
        Self::iterate_diagonal(&grid, &mut result, (0..n).rev().map(|i| (0, i)));

        result
    }

    fn iterate_diagonal<T>(grid: &Vec<Vec<i32>>, result: &mut Vec<Vec<i32>>, iter: T)
    where
        T: Iterator<Item = (usize, usize)>,
    {
        let m = grid.len();
        let n = grid[0].len();
        let max_length = m.min(n);

        let mut length = 1;
        for (i, j) in iter {
            let mut left_set = HashSet::with_capacity(length);
            // 右下对角线因为涉及到添加和删除
            // 使用哈希表存储对应数字的出现次数
            let mut right_set = HashMap::with_capacity(length);

            // 首先遍历右下
            let (mut x, mut y) = (i + 1, j + 1);

            for _ in 1..length {
                let entry = right_set.entry(grid[x][y]).or_insert(0);
                *entry += 1;

                x += 1;
                y += 1;
            }

            // 然后开始遍历
            let (mut x, mut y) = (i, j);

            for k in 0..length {
                // 删除右下对角线
                if k != 0 {
                    let entry = right_set.get_mut(&grid[x][y]).unwrap();
                    *entry -= 1;
                    if *entry == 0 {
                        right_set.remove(&grid[x][y]);
                    }
                }

                result[x][y] = left_set.len().abs_diff(right_set.len()) as i32;

                // 插入左上对角线
                left_set.insert(grid[x][y]);
                x += 1;
                y += 1;
            }

            length = max_length.min(length + 1);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2711() {
        assert_eq!(
            vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]],
            Solution::difference_of_distinct_values(vec![
                vec![1, 2, 3],
                vec![3, 1, 5],
                vec![3, 2, 1]
            ])
        );
        assert_eq!(
            vec![vec![0]],
            Solution::difference_of_distinct_values(vec![vec![1]])
        );
    }
}
