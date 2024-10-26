/**
 * [3181] Maximum Total Reward Using Operations II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut reward_values: Vec<usize> = reward_values.into_iter().map(|x| x as usize).collect();
        reward_values.sort_unstable();
        reward_values.dedup();

        let max_value = *reward_values.last().unwrap();
        let n = reward_values.len();

        if n >= 2 && max_value - 1 == reward_values[n - 2] {
            return (2 * max_value - 1) as i32;
        }

        let mut dp = vec![false; 2 * max_value + 1];
        dp[0] = true;

        unsafe {
            for v in reward_values {
                for i in (v..v << 1).rev() {
                    *(dp.get_unchecked_mut(i)) |= *dp.get_unchecked(i - v);
                }
            }
        }

        dp.iter().enumerate().rfind(|(_, &x)| x).unwrap().0 as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3181() {
        assert_eq!(4, Solution::max_total_reward(vec![1, 1, 3, 3]));
        assert_eq!(11, Solution::max_total_reward(vec![1, 6, 4, 3, 2]));
    }
}
