/**
 * [1884] Egg Drop With 2 Eggs and N Floors
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![usize::MAX / 2; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            for j in 1..=i {
                dp[i] = dp[i].min(j.max(dp[i - j] + 1))
            }
        }

        dp[n] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1884() {
        assert_eq!(2, Solution::two_egg_drop(2));
        assert_eq!(14, Solution::two_egg_drop(100));
    }
}
