/**
 * [1] Two Sum
 *
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * You can return the answer in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 * 	Only one valid answer exists.
 *
 *  
 * Follow-up: Can you come up with an algorithm that is less than O(n^2)<font face="monospace"> </font>time complexity?
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/two-sum/
// discuss: https://leetcode.cn/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (index, value) in nums.iter().enumerate() {
            match map.get(&(target - value)) {
                None => {
                    map.insert(value, index);
                }
                Some(target_index) => {
                    return vec![*target_index as i32, index as i32];
                }
            }
        }

        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
