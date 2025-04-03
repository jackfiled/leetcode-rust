/**
 * [2874] Maximum Value of an Ordered Triplet II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let n = nums.len();
        // max num from nums[0] to nums[i]
        let mut prefix = vec![0; n];
        // max num from nums[i] to nums[n - 1]
        let mut suffix = vec![0; n];

        prefix[0] = nums[0];
        suffix[n - 1] = nums[n - 1];

        for i in 1..n {
            prefix[i] = nums[i].max(prefix[i - 1]);
            suffix[n - i - 1] = nums[n - i - 1].max(suffix[n - i]);
        }

        let mut result = 0;

        for j in 1..n - 1 {
            result = result.max((prefix[j - 1] - nums[j]) * suffix[j + 1]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2874() {
        assert_eq!(77, Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]));
        assert_eq!(133, Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]));
        assert_eq!(0, Solution::maximum_triplet_value(vec![1, 2, 3]));
    }
}
