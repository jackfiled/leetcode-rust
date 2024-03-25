/**
 * [518] Coin Change II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;

        let mut dp = vec![0; amount + 1];
        dp[0] = 1;

        for coin in &coins {
            let coin = *coin as usize;

            for i in coin..=amount {
                dp[i] += dp[i - coin];
            }
        }

        dp[amount]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_518() {
        assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
        assert_eq!(0, Solution::change(3, vec![2]));
        assert_eq!(1, Solution::change(10, vec![10]));
    }
}
