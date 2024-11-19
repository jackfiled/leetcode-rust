/**
 * [3243] Shortest Distance After Road Addition Queries I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut matrix = vec![vec![]; n];
        for i in 0..n - 1 {
            matrix[i].push(i + 1);
        }

        let mut result = vec![];

        for query in queries {
            let (start, end) = (query[0] as usize, query[1] as usize);
            matrix[start].push(end);

            result.push(Self::bfs(n, &matrix));
        }

        result
    }

    fn bfs(n: usize, matrix: &Vec<Vec<usize>>) -> i32 {
        use std::collections::VecDeque;
        let mut distances = vec![-1; n];
        let mut queue = VecDeque::new();
        queue.push_back(0);

        distances[0] = 0;

        while let Some(head) = queue.pop_front() {
            for &next in matrix[head].iter() {
                if distances[next] >= 0 {
                    continue;
                }
                queue.push_back(next);
                distances[next] = distances[head] + 1;
            }
        }

        distances[n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3243() {
        assert_eq!(
            vec![3, 2, 1],
            Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]])
        );
        assert_eq!(
            vec![1, 1],
            Solution::shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]])
        );
    }
}
