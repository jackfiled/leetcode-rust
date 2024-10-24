/**
 * [198] House Robber
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = Vec::with_capacity(n);

        if n == 1 {
            return nums[0];
        }

        let mut result = nums[0];
        dp.push(result);

        for i in 1..n {
            let mut last = 0;

            for j in 0..i - 1 {
                last = last.max(dp[j]);
            }

            last = dp[i - 1].max(last + nums[i]);
            dp.push(last);
            result = result.max(last);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_198() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
        assert_eq!(2, Solution::rob(vec![2, 1]));
        assert_eq!(2, Solution::rob(vec![2]));
        assert_eq!(4, Solution::rob(vec![2, 1, 1, 2]));
    }
}
