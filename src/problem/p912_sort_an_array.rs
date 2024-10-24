/**
 * [912] Sort an Array
 *
 * Given an array of integers nums, sort the array in ascending order and return it.
 * You must solve the problem without using any built-in functions in O(nlog(n)) time complexity and with the smallest space complexity possible.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [5,2,3,1]
 * Output: [1,2,3,5]
 * Explanation: After sorting the array, the positions of some numbers are not changed (for example, 2 and 3), while the positions of other numbers are changed (for example, 1 and 5).
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [5,1,1,2,0,0]
 * Output: [0,0,1,1,2,5]
 * Explanation: Note that the values of nums are not necessairly unique.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^4
 * 	-5 * 10^4 <= nums[i] <= 5 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/sort-an-array/
// discuss: https://leetcode.cn/problems/sort-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut temp = Vec::with_capacity(nums.len());
        temp.resize(nums.len(), 0);
        let (left, right) = (0, nums.len() - 1);

        Solution::merge_sort(&mut nums, &mut temp, left, right);
        nums
    }

    fn merge_sort(nums: &mut Vec<i32>, temp: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = (left + right) / 2;
        Solution::merge_sort(nums, temp, left, mid);
        Solution::merge_sort(nums, temp, mid + 1, right);

        let (mut l, mut r, mut pos) = (left, mid + 1, left);

        while l <= mid && r <= right {
            if nums[l] <= nums[r] {
                temp[pos] = nums[l];
                l += 1;
            } else {
                temp[pos] = nums[r];
                r += 1;
            }
            pos += 1;
        }

        while l <= mid {
            temp[pos] = nums[l];
            pos += 1;
            l += 1;
        }

        while r <= right {
            temp[pos] = nums[r];
            pos += 1;
            r += 1;
        }

        for i in left..=right {
            nums[i] = temp[i];
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_912() {
        assert_eq!(vec![1, 2, 3, 5], Solution::sort_array(vec![5, 2, 3, 1]));
        assert_eq!(
            vec![0, 0, 1, 1, 2, 5],
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0])
        );
    }
}
