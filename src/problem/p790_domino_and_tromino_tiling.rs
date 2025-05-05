/**
 * [790] Domino and Tromino Tiling
 */
pub struct Solution {}

// submission codes start here

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        let mut prefix = 0;

        // dp的最后必须是平整的
        // dp[n] = dp[n - 1] + dp[n - 2] + for i in 0..=n-3 Sum(dp[i] * 2)
        for i in 1..=n {
            dp[i] = dp[i - 1] % MOD;

            if i >= 2 {
                dp[i] = (dp[i] + dp[i - 2]) % MOD;
            }

            if i >= 3 {
                prefix = (prefix + dp[i - 3] * 2 % MOD) % MOD;
                dp[i] = (dp[i] + prefix) % MOD;
            }
        }

        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_790() {
        assert_eq!(1, Solution::num_tilings(1));
        assert_eq!(2, Solution::num_tilings(2));
        assert_eq!(5, Solution::num_tilings(3));
        assert_eq!(11, Solution::num_tilings(4));
        assert_eq!(312342182, Solution::num_tilings(30));
    }
}
