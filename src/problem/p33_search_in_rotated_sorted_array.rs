/**
 * [33] Search in Rotated Sorted Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return if target == nums[0] { 0 } else { -1 };
        }

        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let middle = (right - left) / 2 + left;

            if nums[middle] == target {
                return middle as i32;
            }

            if nums[middle] > nums[0] {
                // 左侧的数组是有序的
                if target >= nums[0] && target < nums[middle] {
                    right = middle;
                } else {
                    left = middle + 1;
                }
            } else {
                // 右侧数组是有序的
                if target > nums[middle] && target <= nums[nums.len() - 1] {
                    left = middle + 1;
                } else {
                    right = middle;
                }
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
    fn test_33() {}
}
