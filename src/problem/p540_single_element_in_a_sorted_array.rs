/**
 * [540] Single Element in a Sorted Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut left, mut right) = (0, n - 1);

        while left < right {
            let middle = (right - left) / 2 + left;
            if nums[middle] == nums[middle ^ 1] {
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
    fn test_540() {
        assert_eq!(
            2,
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])
        );
        assert_eq!(
            10,
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])
        );
    }
}
