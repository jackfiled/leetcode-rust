/**
 * [2218] Maximum Value of K Coins From Piles
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![-1; k + 1];
        dp[0] = 0;

        for pile in piles {
            for i in (1..=k).rev() {
                let mut value = 0;

                for t in 1..=pile.len() {
                    value += pile[t - 1];

                    if i >= t && dp[i - t] != -1 {
                        dp[i] = dp[i].max(dp[i - t] + value);
                    }
                }
            }
        }

        dp[k]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2218() {
        assert_eq!(
            101,
            Solution::max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2)
        );
    }
}
