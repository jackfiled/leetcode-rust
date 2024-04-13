/**
 * [45] Jump Game II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; nums.len()];
        dp[0] = 0;

        for i in 0..nums.len() {
            if dp[i] == i32::MAX {
                continue;
            }

            for j in 1..=nums[i] as usize {
                if i + j >= nums.len() {
                    break;
                }

                dp[i + j] = dp[i + j].min(dp[i] + 1);
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
    fn test_45() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    }
}
