/**
 * [918] Maximum Sum Circular Subarray
 */
pub struct Solution {}


// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut result = nums[0];
        let mut queue = VecDeque::new();
        let mut pre = nums[0];
        queue.push_back((0, pre));

        for i in 1..(2 * n) {
            while !queue.is_empty() && queue.front().unwrap().0 + n < i {
                queue.pop_front();
            }

            pre += nums[i % n];
            result = result.max(pre - queue.front().unwrap().1);

            while !queue.is_empty() && queue.back().unwrap().1 >= pre {
                queue.pop_back();
            }
            queue.push_back((i, pre));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_918() {
    }
}
