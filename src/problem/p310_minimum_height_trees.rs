/**
 * [310] Minimum Height Trees
 */
pub struct Solution {}


// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![];n];

        for edge in &edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            graph[x].push(y);
            graph[y].push(x);
        }

        let mut parents = vec![usize::MAX;n];
        let x = Solution::find_longest_node(0, &mut parents, &graph);
        let mut y = Solution::find_longest_node(x, &mut parents, &graph);

        let mut path = Vec::new();
        parents[x] = usize::MAX;

        parents = dbg!(parents);
        while y != usize::MAX {
            path.push(y);
            y = parents[y];
        }

        let length = path.len();

        return if length % 2 != 0 {
            vec![path[length / 2] as i32]
        } else {
            vec![path[length / 2 - 1] as i32, path[length / 2] as i32]
        };
    }

    fn find_longest_node(node: usize, parents: &mut Vec<usize>, graph: &Vec<Vec<usize>>) -> usize {
        let n = graph.len();

        let mut queue = VecDeque::with_capacity(n);
        let mut visited = vec![false;n];

        queue.push_back(node);
        visited[node] = true;
        let mut result = usize::MAX;

        while !queue.is_empty() {
            let now = queue.pop_front().unwrap();
            dbg!(now);
            result = now;
            for next in &graph[now] {
                let next = *next;
                if !visited[next] {
                    visited[next] = true;
                    parents[next] = now;
                    queue.push_back(next);
                }
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
    fn test_310() {
        assert_eq!(vec![1], Solution::find_min_height_trees(4, vec![vec![1,0], vec![1,2], vec![1,3]]));
    }
}
