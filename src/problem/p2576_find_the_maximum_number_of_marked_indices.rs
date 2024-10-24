/**
 * [2576] Find the Maximum Number of Marked Indices
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_num_of_marked_indices(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();

        let check = |k: usize| -> bool {
            for i in 0..k {
                if nums[i] * 2 > nums[n - k + i] {
                    return false;
                }
            }

            true
        };

        let (mut left, mut right) = (0, n / 2);

        while left < right {
            let middle = (left + right + 1) / 2;

            if check(middle) {
                left = middle;
            } else {
                right = middle - 1;
            }
        }

        (left * 2) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2576() {
        assert_eq!(2, Solution::max_num_of_marked_indices(vec![3, 5, 2, 4]));
        assert_eq!(4, Solution::max_num_of_marked_indices(vec![9, 2, 5, 4]));
        assert_eq!(0, Solution::max_num_of_marked_indices(vec![7, 6, 8]));
    }
}
