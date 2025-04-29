/**
 * [2302] Count Subarrays With Score Less Than K
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();

        let (mut result, mut total_sum) = (0, 0);
        let mut left = 0;

        for right in 0..n {
            total_sum += nums[right] as i64;

            while left <= right && total_sum * ((right - left + 1) as i64) >= k {
                total_sum -= nums[left] as i64;
                left += 1;
            }

            result += (right as i64 - left as i64) + 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2302() {
        assert_eq!(
            3,
            Solution::count_subarrays(
                vec![9, 5, 3, 8, 4, 7, 2, 7, 4, 5, 4, 9, 1, 4, 8, 10, 8, 4, 7],
                4
            )
        );
        assert_eq!(6, Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10));
        assert_eq!(5, Solution::count_subarrays(vec![1, 1, 1], 5));
    }
}
