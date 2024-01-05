/**
 * [743] Network Delay Time
 *
 * You are given a network of n nodes, labeled from 1 to n. You are also given times, a list of travel times as directed edges times[i] = (ui, vi, wi), where ui is the source node, vi is the target node, and wi is the time it takes for a signal to travel from source to target.
 * We will send a signal from a given node k. Return the minimum time it takes for all the n nodes to receive the signal. If it is impossible for all the n nodes to receive the signal, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/23/931_example_1.png" style="width: 217px; height: 239px;" />
 * Input: times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2
 * Output: 2
 * 
 * <strong class="example">Example 2:
 * 
 * Input: times = [[1,2,1]], n = 2, k = 1
 * Output: 1
 * 
 * <strong class="example">Example 3:
 * 
 * Input: times = [[1,2,1]], n = 2, k = 2
 * Output: -1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= n <= 100
 * 	1 <= times.length <= 6000
 * 	times[i].length == 3
 * 	1 <= ui, vi <= n
 * 	ui != vi
 * 	0 <= wi <= 100
 * 	All the pairs (ui, vi) are unique. (i.e., no multiple edges.)
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/network-delay-time/
// discuss: https://leetcode.cn/problems/network-delay-time/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::{max, min};
use std::collections::HashSet;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let mut matrix = Vec::with_capacity(n + 1);

        for _i in 0..=n {
            let mut line = Vec::with_capacity(n + 1);
            line.resize(n + 1, i32::MAX);
            matrix.push(line);
        }

        for line in &times {
            matrix[line[0] as usize][line[1] as usize] = line[2];
        }

        let mut path = HashSet::with_capacity(n + 1);
        path.insert(k);
        let mut distances = matrix[k].clone();

        for _ in 2..=n {
            let (mut index, mut value) = (0usize, i32::MAX);

            for (i, d) in distances.iter().enumerate() {
                if path.contains(&i) {
                    continue;
                }

                if value > *d {
                    value = *d;
                    index = i;
                }
            }

            if value == i32::MAX {
                break;
            }

            path.insert(index);

            for i in 1..=n {
                if path.contains(&i) {
                    continue;
                }

                if matrix[index][i] == i32::MAX {
                    continue;
                }

                distances[i] = min(distances[i], value + matrix[index][i]);
            }
        }

        let mut result = i32::MIN;

        for i in 1..distances.len() {
            if i == k {
                continue
            }

            result = max(result, distances[i]);
        }

        return if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_743() {
        assert_eq!(Solution::network_delay_time(
            vec![vec![2,1,1], vec![2,3,1], vec![3,4,1]], 4, 2), 2);
        assert_eq!(Solution::network_delay_time(vec![vec![1,2,1]], 2, 1), 1);
        assert_eq!(Solution::network_delay_time(vec![vec![1,2,1]], 2, 2), -1);
    }
}
