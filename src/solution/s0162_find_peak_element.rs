/**
 * [162] Find Peak Element
 *
 * A peak element is an element that is strictly greater than its neighbors.
 * Given a 0-indexed integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.
 * You may imagine that nums[-1] = nums[n] = -&infin;. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.
 * You must write an algorithm that runs in O(log n) time.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,2,3,1]
 * Output: 2
 * Explanation: 3 is a peak element and your function should return the index number 2.
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,2,1,3,5,6,4]
 * Output: 5
 * Explanation: Your function can return either index number 1 where the peak element is 2, or index number 5 where the peak element is 6.
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	nums[i] != nums[i + 1] for all valid i.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/find-peak-element/
// discuss: https://leetcode.cn/problems/find-peak-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            0
        }

        let (mut left, mut right) = (0, nums.len() - 1);

        let compare = |x| {
            if x == 0 {
                nums[0] > nums[1]
            } else if x == nums.len() - 1 {
                nums[x] > nums[x - 1]
            } else {
                nums[x - 1] < nums[x] && nums[x] > nums[x + 1]
            }
        };

        while left <= right {
            let mid = (left + right) / 2;

            if compare(mid) {
                return mid as i32
            }

            if nums[mid] < nums[mid + 1] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_162() {
        assert_eq!(2, Solution::find_peak_element(vec![1,2,3,1]));
        assert_eq!(5, Solution::find_peak_element(vec![1,2,1,3,5,6,4]));
        assert_eq!(1, Solution::find_peak_element(vec![1,2]));
    }
}
