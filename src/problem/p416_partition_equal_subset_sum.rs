/**
 * [416] Partition Equal Subset Sum
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 2 {
            return false;
        }

        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
        let sum: usize = nums.iter().sum();

        if sum % 2 != 0 {
            return false;
        }

        let middle = sum / 2;
        let mut dp = vec![vec![false; middle + 1]; n];

        // 初始化dp
        for i in 0..n {
            dp[i][0] = true;
        }
        if nums[0] <= middle {
            dp[0][nums[0]] = true;
        }

        for i in 1..n {
            for j in 1..=middle {
                if j < nums[i] {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i]];
                }
            }
        }

        dp[n - 1][middle]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_416() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    }
}
