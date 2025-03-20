/**
 * [2612] Minimum Reverse Operations
 */
pub struct Solution {}

// submission codes start here
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::iter::FromIterator;

impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        let (n, p) = (n as usize, p as usize);
        let mut ban_pos: HashSet<usize> =
            HashSet::from_iter(banned.into_iter().map(|x| x as usize));
        // 奇数下标和偶数下标是分别连续的
        // 使用两个二叉树维护
        let mut odd_pos = BTreeSet::new();
        let mut even_pos = BTreeSet::new();

        for i in 0..n {
            if i != p && !ban_pos.contains(&i) {
                if i % 2 == 0 {
                    even_pos.insert(i);
                } else {
                    odd_pos.insert(i);
                }
            }
        }

        let mut result = vec![-1; n];
        let mut queue = VecDeque::new();
        queue.push_back(p);
        result[p] = 0;

        while let Some(front) = queue.pop_front() {
            // 为了防止usize溢出的诡异类型转换
            let min_pos = (front as i32 - k + 1).max(k - front as i32 - 1) as usize;
            let max_pos = (front as i32 + k - 1).min(n as i32 * 2 - k - front as i32 - 1) as usize;

            let mut iter = if max_pos % 2 == 0 {
                even_pos.range(min_pos..)
            } else {
                odd_pos.range(min_pos..)
            };

            while let Some(&value) = iter.next() {
                if value > max_pos {
                    break;
                }

                result[value] = result[front] + 1;
                queue.push_back(value);

                iter = if min_pos % 2 == 0 {
                    even_pos.remove(&value);
                    even_pos.range(value + 1..)
                } else {
                    odd_pos.remove(&value);
                    odd_pos.range(value + 1..)
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
    fn test_2612() {
        assert_eq!(
            vec![0, -1, -1, 1],
            Solution::min_reverse_operations(4, 0, vec![1, 2], 4)
        );
        assert_eq!(
            vec![0, -1, -1, -1, -1],
            Solution::min_reverse_operations(5, 0, vec![2, 4], 3)
        );
        assert_eq!(
            vec![-1, -1, 0, -1],
            Solution::min_reverse_operations(4, 2, vec![0, 1, 3], 1)
        );
    }
}
