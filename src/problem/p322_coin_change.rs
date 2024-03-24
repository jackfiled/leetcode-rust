/**
 * [322] Coin Change
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;

        for i in 1..=amount {
            for coin in &coins {
                let coin = *coin as usize;
                if coin <= i && dp[i - coin] != i32::MAX {
                    dp[i] = dp[i].min(dp[i - coin] + 1);
                }
            }
        }

        return if dp[amount] == i32::MAX {
            -1
        } else {
            dp[amount]
        };
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_322() {
        assert_eq!(3, Solution::coin_change(vec![1,2,5], 11));
        assert_eq!(-1, Solution::coin_change(vec![2], 3));
        assert_eq!(0, Solution::coin_change(vec![1], 0));
    }
}
