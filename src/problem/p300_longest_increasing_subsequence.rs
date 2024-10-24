/**
 * [300] Longest Increasing Subsequence
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut length = 1;
        let mut dp = vec![0; n + 1];
        dp[length] = nums[0];

        for i in 1..n {
            if nums[i] > dp[length] {
                length += 1;
                dp[length] = nums[i];
            } else {
                // 二分查找dp中小于nums[i]的位置
                let (mut left, mut right, mut pos) = (1, length, 0);
                while left <= right {
                    let middle = (right - left) / 2 + left;
                    if dp[middle] < nums[i] {
                        pos = middle;
                        left = middle + 1;
                    } else {
                        right = middle - 1;
                    }
                }

                dp[pos + 1] = nums[i];
            }
        }

        length as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_300() {}
}
