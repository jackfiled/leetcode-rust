/**
 * [1976] Number of Ways to Arrive at Destination
 */
pub struct Solution {}

// submission codes start here
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Node {
    node: usize,
    distance: i64,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let m = 1e9 as i32 + 7;
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for road in roads {
            let x = road[0] as usize;
            let y = road[1] as usize;
            let t = road[2] as i64;

            graph[x].push((y, t));
            graph[y].push((x, t));
        }

        let mut distance = vec![i64::MAX; n];
        distance[0] = 0;
        let mut ways = vec![0; n];
        ways[0] = 1;

        let mut queue = BinaryHeap::new();
        queue.push(Node {
            node: 0,
            distance: 0,
        });

        while !queue.is_empty() {
            let now = queue.pop().unwrap();

            if now.distance > distance[now.node] {
                continue;
            }

            for next in &graph[now.node] {
                if now.distance + next.1 < distance[next.0] {
                    distance[next.0] = now.distance + next.1;
                    ways[next.0] = ways[now.node];

                    queue.push(Node {
                        node: next.0,
                        distance: now.distance + next.1,
                    });
                } else if now.distance + next.1 == distance[next.0] {
                    ways[next.0] = (ways[now.node] + ways[next.0]) % m;
                }
            }
        }

        ways[n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1976() {}
}
