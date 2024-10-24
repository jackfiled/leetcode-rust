/**
 * [1696] Jump Game VI
 */
pub struct Solution {}

// submission codes start here

use std::collections::VecDeque;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut dp = vec![0; n];
        let mut queue = VecDeque::new();

        dp[0] = nums[0];
        queue.push_back(0);

        for i in 1..n {
            while let Some(h) = queue.front() {
                if i - *h > k {
                    queue.pop_front();
                } else {
                    break;
                }
            }

            dp[i] = dp[*queue.front().unwrap()] + nums[i];
            while let Some(tail) = queue.back() {
                if dp[*tail] <= dp[i] {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back(i);
        }

        dp[n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1696() {
        assert_eq!(Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
        assert_eq!(Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
        assert_eq!(
            Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2),
            0
        );
    }
}
