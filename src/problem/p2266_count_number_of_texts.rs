/**
 * [2266] Count Number of Texts
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

const MODE: i64 = 1_000_000_007;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let three_keys = HashSet::from(['2', '3', '4', '5', '6', '8']);
        let pressed_keys: Vec<char> = pressed_keys.chars().collect();

        let mut three_dp = Vec::from([0, 1, 2, 4]);
        let mut four_dp = Vec::from([0, 1, 2, 4, 8]);

        let mut get_three = |n: usize| {
            if n >= three_dp.len() {
                for i in three_dp.len()..=n {
                    three_dp.push((three_dp[i - 1] + three_dp[i - 2] + three_dp[i - 3]) % MODE);
                }
            }

            three_dp[n]
        };

        let mut get_four = |n: usize| {
            if n >= four_dp.len() {
                for i in four_dp.len()..=n {
                    four_dp.push(
                        (four_dp[i - 1] + four_dp[i - 2] + four_dp[i - 3] + four_dp[i - 4]) % MODE,
                    );
                }
            }

            four_dp[n]
        };

        let mut last_key = None;
        let mut count = 0;
        let mut result = 1;

        for c in pressed_keys {
            if let Some(last) = last_key {
                if last == c {
                    count += 1;
                } else {
                    if three_keys.contains(&last) {
                        result = result * get_three(count) % MODE;
                    } else {
                        result = result * get_four(count) % MODE;
                    }

                    last_key = Some(c);
                    count = 1;
                }
            } else {
                last_key = Some(c);
                count = 1;
            }
        }

        if let Some(last) = last_key {
            if three_keys.contains(&last) {
                result = result * get_three(count) % MODE;
            } else {
                result = result * get_four(count) % MODE;
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
    fn test_2266() {
        assert_eq!(3136, Solution::count_texts("444479999555588866".to_owned()));
        assert_eq!(8, Solution::count_texts("22233".to_owned()));
        assert_eq!(
            82876089,
            Solution::count_texts("222222222222222222222222222222222222".to_owned())
        );
    }
}
