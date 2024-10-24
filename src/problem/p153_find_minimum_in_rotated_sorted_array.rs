/**
 * [153] Find Minimum in Rotated Sorted Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut left, mut right) = (0, n);

        while left < right {
            let middle = (right - left) / 2 + left;

            if nums[middle] > nums[n - 1] {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        nums[left]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_153() {}
}
