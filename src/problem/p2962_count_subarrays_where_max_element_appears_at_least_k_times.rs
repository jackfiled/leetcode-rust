/**
 * [2962] Count Subarrays Where Max Element Appears at Least K Times
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();
        let n = nums.len();

        let mut result = 0;
        let mut count = 0;
        let mut left = 0;

        for right in 0..n {
            count += if nums[right] == max { 1 } else { 0 };

            if count >= k {
                result += (n - right) as i64;
            }

            while left <= right && count >= k {
                count -= if nums[left] == max { 1 } else { 0 };

                if count >= k {
                    result += (n - right) as i64;
                }
                left += 1;
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
    fn test_2962() {
        assert_eq!(6, Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2));
        assert_eq!(0, Solution::count_subarrays(vec![1, 4, 2, 1], 3));
    }
}
