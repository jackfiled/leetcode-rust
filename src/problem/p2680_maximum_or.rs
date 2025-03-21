/**
 * [2680] Maximum OR
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as i64;
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];

        for i in 0..n {
            if i == 0 {
                prefix[i] = 0;
                suffix[n - i - 1] = 0;
            } else {
                prefix[i] = prefix[i - 1] | nums[i - 1] as i64;
                suffix[n - i - 1] = suffix[n - i] | nums[n - i] as i64;
            }
        }

        let mut result = 0;

        for (i, &v) in nums.iter().enumerate() {
            result = result.max(prefix[i] | ((v as i64) << k) | suffix[i]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2680() {
        assert_eq!(30, Solution::maximum_or(vec![12, 9], 1));
        assert_eq!(35, Solution::maximum_or(vec![8, 1, 2], 2));
    }
}
