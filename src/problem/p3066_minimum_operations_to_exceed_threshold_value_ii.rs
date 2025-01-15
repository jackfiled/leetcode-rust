/**
 * [3066] Minimum Operations to Exceed Threshold Value II
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut heap = BinaryHeap::from_iter(nums.into_iter().map(|x| Reverse(x as i64)));

        let mut result = 0;

        while let Some(head) = heap.peek() {
            if head.0 >= k {
                break;
            }

            let first = heap.pop().unwrap().0;
            let second = heap.pop().unwrap().0;

            heap.push(Reverse(first.min(second) * 2 + first.max(second)));
            result += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3066() {
        assert_eq!(
            2,
            Solution::min_operations(vec![999999999, 999999999, 999999999], 1_000_000_000)
        );
        assert_eq!(2, Solution::min_operations(vec![2, 11, 10, 1, 3], 10));
        assert_eq!(4, Solution::min_operations(vec![1, 1, 2, 4, 9], 20));
    }
}
