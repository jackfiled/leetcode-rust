use std::convert::TryInto;

/**
 * [2359] Find Closest Node to Given Two Nodes
 */
pub struct Solution {}

// submission codes start here
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let edges: Vec<Option<usize>> = edges.into_iter().map(|x| x.try_into().ok()).collect();

        let distances1 = Self::search_distances(&edges, node1 as usize);
        let distances2 = Self::search_distances(&edges, node2 as usize);

        let mut result: Option<(usize, i32)> = None;

        for i in 0..edges.len() {
            if distances1[i] == i32::MAX || distances2[i] == i32::MAX {
                continue;
            }

            let distance = distances1[i].max(distances2[i]);

            if let Some((pos, d)) = result {
                if distance < d {
                    result = Some((i, distance));
                }
            } else {
                result = Some((i, distance));
            }
        }

        result.map_or(-1, |x| x.0 as i32)
    }

    fn search_distances(edges: &Vec<Option<usize>>, node: usize) -> Vec<i32> {
        let mut distances = vec![i32::MAX; edges.len()];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((node, 0));

        while let Some((pos, d)) = queue.pop_front() {
            if !visited.insert(pos) {
                continue;
            }

            distances[pos] = d;

            if let Some(next) = edges[pos] {
                queue.push_back((next, d + 1));
            }
        }

        distances
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2359() {
        assert_eq!(2, Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1));
        assert_eq!(2, Solution::closest_meeting_node(vec![1, 2, -1], 0, 2));
    }
}
