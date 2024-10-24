/**
 * [410] Split Array Largest Sum
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn check(nums: &Vec<i32>, value: i64, m: i32) -> bool {
        let mut sum = 0i64;
        let mut count = 1;

        for i in nums {
            let i = *i as i64;

            if i + sum > value {
                sum = i;
                count += 1;
            } else {
                sum += i;
            }
        }

        count <= m
    }

    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut right = 0i64;
        let mut left = i64::MIN;

        for i in &nums {
            let i = *i as i64;

            if i > left {
                left = i;
            }
            right += i;
        }

        while left < right {
            let mid = (left + right) / 2;

            if Solution::check(&nums, mid, k) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_410() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
    }
}
