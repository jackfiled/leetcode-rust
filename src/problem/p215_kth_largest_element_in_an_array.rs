/**
 * [215] Kth Largest Element in an Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        use std::{cmp::Reverse, collections::BinaryHeap};

        let k = k as usize;
        let mut heap = BinaryHeap::<Reverse<i32>>::with_capacity(k);

        for i in nums {
            if heap.len() == k && heap.peek().unwrap().0 < i {
                heap.pop();
                heap.push(Reverse(i));
            } else if heap.len() < k {
                heap.push(Reverse(i));
            }
        }

        heap.pop().unwrap().0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_215() {}
}
