/**
 * [2369] Check if There is a Valid Partition For The Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut dp = vec![false; nums.len()];
        dp[1] = nums[0] == nums[1];

        for i in 2..nums.len() {
            // 首先判断用两个相同的值
            dp[i] = dp[i - 2] && nums[i] == nums[i - 1];

            if dp[i] {
                continue;
            }

            // 然后判断三个相同的值
            if i >= 3 {
                dp[i] = dp[i - 3]
                    && ((nums[i - 2] == nums[i - 1] && nums[i - 1] == nums[i])
                        || (nums[i - 2] + 1 == nums[i - 1] && nums[i - 1] + 1 == nums[i]));
            } else {
                dp[i] = (nums[i - 2] == nums[i - 1] && nums[i - 1] == nums[i])
                    || (nums[i - 2] + 1 == nums[i - 1] && nums[i - 1] + 1 == nums[i]);
            }
        }

        dp[nums.len() - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2369() {
        assert!(Solution::valid_partition(vec![4, 4, 4, 5, 6]));
        assert!(!Solution::valid_partition(vec![4, 4, 4, 5]));
        assert!(Solution::valid_partition(vec![
            803201, 803201, 803201, 803201, 803202, 803203
        ]));
        assert!(!Solution::valid_partition(vec![
            993335, 993336, 993337, 993338, 993339, 993340, 993341
        ]));
    }
}
