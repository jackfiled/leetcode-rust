/**
 * [1857] Largest Color Value in a Directed Graph
 */
pub struct Solution {}

// submission codes start here
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let colors: Vec<usize> = colors.bytes().map(|x| (x - b'a') as usize).collect();
        let mut adjacency_matrix = vec![vec![]; colors.len()];
        // 入度节点
        let mut in_degrees = vec![0; colors.len()];

        for edge in edges.iter() {
            let start = edge[0] as usize;
            let end = edge[1] as usize;

            adjacency_matrix[start].push(end);
            in_degrees[end] += 1;
        }

        // 使用拓扑排序判断是否有环
        // 同时获得一个拓扑序列表
        let mut queue = VecDeque::from_iter(in_degrees.iter().enumerate().filter_map(|(i, v)| {
            if *v == 0 {
                Some(i)
            } else {
                None
            }
        }));
        let mut topology_list = vec![];

        while let Some(head) = queue.pop_front() {
            topology_list.push(head);
            for &next in adjacency_matrix[head].iter() {
                in_degrees[next] -= 1;

                if in_degrees[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if in_degrees.iter().any(|x| *x != 0) {
            return -1;
        }

        // dp[i][c]表示从节点i开始路径上颜色为c的计数
        // 颜色只能是小写字母
        let mut dp = vec![vec![0; 26]; topology_list.len()];
        let mut result = HashMap::new();

        for &node in topology_list.iter().rev() {
            let c = colors[node];

            for &next in adjacency_matrix[node].iter() {
                for i in 0..26 {
                    dp[node][i] = dp[node][i].max(dp[next][i]);
                }

                dp[node][c] = dp[node][c].max(dp[next][c] + 1);
                let entry = result.entry(c).or_insert(0);
                *entry = dp[node][c].max(*entry);
            }

            if adjacency_matrix[node].is_empty() {
                dp[node][c] = 1;
            }
        }

        result.values().max().map_or_else(|| 1, |x| *x)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1857() {
        assert_eq!(
            3,
            Solution::largest_path_value(
                "abaca".to_string(),
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]]
            )
        );
        assert_eq!(
            -1,
            Solution::largest_path_value("a".to_string(), vec![vec![0, 0]])
        );
    }
}
