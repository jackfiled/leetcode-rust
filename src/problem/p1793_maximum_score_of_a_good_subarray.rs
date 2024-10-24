/**
 * [1793] Maximum Score of a Good Subarray
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let length = nums.len() as i32;

        let mut left = k - 1;
        let mut right = k + 1;

        let mut result = 0;

        for i in (0..=nums[k as usize]).rev() {
            while left >= 0 && nums[left as usize] >= i {
                left -= 1;
            }

            while right < length && nums[right as usize] >= i {
                right += 1;
            }

            result = result.max((right - left - 1) * i);

            if left == -1 && right == length {
                break;
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
    fn test_1793() {
        assert_eq!(15, Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3));
    }
}
