/**
 * [743] Network Delay Time
 */
pub struct Solution {}

// submission codes start here
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

                distances[i] = distances[i].min(value + matrix[index][i]);
            }
        }

        let mut result = i32::MIN;

        for i in 1..distances.len() {
            if i == k {
                continue;
            }

            result = result.max(distances[i]);
        }

        if result == i32::MAX {
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
        assert_eq!(
            Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
            2
        );
        assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
        assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
    }
}
