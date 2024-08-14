/**
 * [188] Best Time to Buy and Sell Stock IV
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = k as usize;

        let mut dp = vec![(-prices[0], 0); k];

        for i in 1..n {
            for j in 0..k {
                dp[j].0 = dp[j].0.max(if j == 0 {
                    -prices[i]
                } else {
                    dp[j - 1].1 - prices[i]
                });
                dp[j].1 = dp[j].1.max(dp[j].0 + prices[i]);
            }
        }

        dp[k - 1].1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_188() {
        assert_eq!(2, Solution::max_profit(2, vec![2, 4, 1]));
        assert_eq!(7, Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]));
    }
}
