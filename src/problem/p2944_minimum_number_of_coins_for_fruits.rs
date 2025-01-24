/**
 * [2944] Minimum Number of Coins for Fruits
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_coins(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        // (滑动窗口的右侧, dp[i])
        let mut queue = VecDeque::from([(n + 1, 0)]);

        for i in (1..=n).rev() {
            while queue.back().unwrap().0 > i * 2 + 1 {
                queue.pop_back();
            }

            let dp = prices[i - 1] + queue.back().unwrap().1;
            while dp <= queue.front().unwrap().1 {
                queue.pop_front();
            }

            queue.push_front((i, dp));
        }

        queue.front().unwrap().1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2944() {
        assert_eq!(4, Solution::minimum_coins(vec![3, 1, 2]));
        assert_eq!(2, Solution::minimum_coins(vec![1, 10, 1, 1]));
        assert_eq!(
            39,
            Solution::minimum_coins(vec![26, 18, 6, 12, 49, 7, 45, 45])
        );
    }
}
