/**
 * [2642] Design Graph With Shortest Path Calculator
 */
pub struct Solution {}

// submission codes start here
use std::{cmp::Reverse, collections::BinaryHeap};

struct Graph {
    graph: Vec<Vec<(usize, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;

        let mut graph = vec![vec![]; n];

        for edge in &edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            graph[x].push((y, edge[2]));
        }

        Graph { graph }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let x = edge[0] as usize;
        let y = edge[1] as usize;

        self.graph[x].push((y, edge[2]));
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let (start, end) = (node1 as usize, node2 as usize);
        let mut queue = BinaryHeap::new();
        let mut distances = vec![i32::MAX; self.graph.len()];
        distances[start] = 0;
        queue.push((Reverse(0), start));

        while !queue.is_empty() {
            let (cost, now) = queue.pop().unwrap();

            if now == end {
                return cost.0;
            }

            for &(next, dis) in &self.graph[now] {
                if distances[next] > cost.0 + dis {
                    distances[next] = cost.0 + dis;
                    queue.push((Reverse(distances[next]), next));
                }
            }
            
        }

        -1
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2642() {}
}
