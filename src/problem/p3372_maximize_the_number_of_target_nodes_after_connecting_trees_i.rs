/**
 * [3372] Maximize the Number of Target Nodes After Connecting Trees I
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // 首先构建邻接矩阵
        let adjacent_matrix1 = Self::construct_adjacent_matrix(&edges1);
        let adjacent_matrix2 = Self::construct_adjacent_matrix(&edges2);

        let target_pointer_counts1 = Self::calculate_target_pointer_counts(&adjacent_matrix1, k);
        let target_pointer_counts2 =
            Self::calculate_target_pointer_counts(&adjacent_matrix2, k - 1);
        let max_count = *target_pointer_counts2.iter().max().unwrap();

        target_pointer_counts1
            .iter()
            .map(|&c| c + max_count)
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

    fn calculate_target_pointer_counts(
        adjacent_matrix: &Vec<Vec<usize>>,
        distance: i32,
    ) -> Vec<i32> {
        let mut queue = VecDeque::new();

        (0..adjacent_matrix.len())
            .map(|i| {
                let mut count = 0;

                queue.clear();
                if distance >= 0 {
                    queue.push_back((i, 0, None));
                }

                while let Some((pos, c, parent)) = queue.pop_front() {
                    count += 1;

                    if c + 1 <= distance {
                        for &next in adjacent_matrix[pos].iter() {
                            if parent.filter(|x| x == &next).is_some() {
                                continue;
                            }

                            queue.push_back((next, c + 1, Some(pos)));
                        }
                    }
                }

                count
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3372() {
        assert_eq!(
            vec![1, 1],
            Solution::max_target_nodes(vec![vec![0, 1]], vec![vec![0, 1]], 0)
        );
        assert_eq!(
            vec![9, 7, 9, 8, 8],
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
                ],
                2
            )
        );
        assert_eq!(
            vec![6, 3, 3, 3, 3],
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                1
            )
        );
    }
}
