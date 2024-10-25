/**
 * [3180] Maximum Total Reward Using Operations I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut reward_values: Vec<usize> = reward_values.into_iter().map(|x| x as usize).collect();
        reward_values.sort();

        let max_value = *reward_values.last().unwrap();
        let mut dp = vec![false; max_value * 2];
        dp[0] = true;

        for &x in reward_values.iter() {
            for i in (x..=(2 * x - 1)).rev() {
                if dp[i - x] {
                    dp[i] = true;
                }
            }
        }

        let mut result = 0;

        for (i, &x) in dp.iter().enumerate() {
            if x {
                result = i;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3180() {
        assert_eq!(4, Solution::max_total_reward(vec![1, 1, 3, 3]));
        assert_eq!(11, Solution::max_total_reward(vec![1, 6, 4, 3, 2]));
    }
}
