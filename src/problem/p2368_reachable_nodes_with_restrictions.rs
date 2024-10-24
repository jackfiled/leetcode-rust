/**
 * [2368] Reachable Nodes With Restrictions
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut result = 0;
        let n = n as usize;
        let mut graph = vec![vec![]; n + 1];

        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            graph[x].push(y);
            graph[y].push(x);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut unreachable = HashSet::with_capacity(restricted.len());

        for node in restricted {
            unreachable.insert(node as usize);
        }

        queue.push_back(0);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if visited.contains(&node) || unreachable.contains(&node) {
                continue;
            }

            visited.insert(node);
            result += 1;

            for next in &graph[node] {
                queue.push_back(*next);
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
    fn test_2368() {}
}
