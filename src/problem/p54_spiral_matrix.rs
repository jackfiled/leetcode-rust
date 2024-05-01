/**
 * [54] Spiral Matrix
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (height, width) = (matrix.len() as i32, matrix[0].len() as i32);
        let mut result = Vec::with_capacity((height * width) as usize);
        let mut matrix = matrix;

        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut pos: (i32, i32) = (0, 0);
        let mut direction = 0;

        loop {
            result.push(matrix[pos.0 as usize][pos.1 as usize]);
            matrix[pos.0 as usize][pos.1 as usize] = i32::MAX;

            let mut flag = true;
            let mut count = 0;
            while flag {
                if count > 4 {
                    break;
                }

                let next_pos = (
                    pos.0 + directions[direction].0,
                    pos.1 + directions[direction].1,
                );

                if next_pos.0 < 0 || next_pos.0 >= height || next_pos.1 < 0 || next_pos.1 >= width {
                    direction = (direction + 1) % 4;
                    count += 1;
                    continue;
                } else {
                    if (matrix[next_pos.0 as usize][next_pos.1 as usize] == i32::MAX) {
                        direction = (direction + 1) % 4;
                        count += 1;
                        continue;
                    }
                }

                pos = next_pos;
                flag = false;
            }

            if flag {
                break;
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
    fn test_54() {
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ])
        );
    }
}
