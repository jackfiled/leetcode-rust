/**
 * [3373] Maximize the Number of Target Nodes After Connecting Trees II
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let adjacent_matrix1 = Self::construct_adjacent_matrix(&edges1);
        let adjacent_matrix2 = Self::construct_adjacent_matrix(&edges2);

        let (even_count1, odd_count1, color1) =
            Self::calculate_target_pointer_counts(&adjacent_matrix1);
        let (even_count2, odd_count2, _) = Self::calculate_target_pointer_counts(&adjacent_matrix2);

        let max_count = odd_count2.max(even_count2);

        (0..adjacent_matrix1.len())
            .map(|i| {
                if color1[i] {
                    even_count1 + max_count
                } else {
                    odd_count1 + max_count
                }
            })
            .collect()
    }

    fn construct_adjacent_matrix(edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let n = edges.len() + 1;
        let mut adjacent_matrix = vec![vec![]; n];

        for edge in edges.iter() {
            let first = edge[0] as usize;
            let second = edge[1] as usize;

            adjacent_matrix[first].push(second);
            adjacent_matrix[second].push(first);
        }

        adjacent_matrix
    }

    fn calculate_target_pointer_counts(adjacent_matrix: &Vec<Vec<usize>>) -> (i32, i32, Vec<bool>) {
        let mut queue = VecDeque::new();
        // 染色数组
        // true表示距离为偶数
        // false表示距离为奇数
        let mut color = vec![false; adjacent_matrix.len()];
        // 距离为偶数的计数
        let mut result = 0;

        queue.push_back((0, 0, None));

        while let Some((pos, c, parent)) = queue.pop_front() {
            if c % 2 == 0 {
                color[pos] = true;
                result += 1;
            }

            for &next in adjacent_matrix[pos].iter() {
                if parent.filter(|x| x == &next).is_some() {
                    continue;
                }

                queue.push_back((next, c + 1, Some(pos)));
            }
        }

        (result, adjacent_matrix.len() as i32 - result, color)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3373() {
        assert_eq!(
            vec![8, 7, 7, 8, 8],
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 7],
                    vec![1, 4],
                    vec![4, 5],
                    vec![4, 6]
                ]
            )
        );
        assert_eq!(
            vec![3, 6, 6, 6, 6],
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]]
            )
        );
    }
}
