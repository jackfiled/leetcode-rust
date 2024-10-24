/**
 * [514] Freedom Trail
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut map = HashMap::new();

        for (index, value) in ring.chars().enumerate() {
            let entry = map.entry(value).or_insert(Vec::new());
            entry.push(index);
        }

        let mut dp = Vec::with_capacity(key.len());
        for _ in 0..key.len() {
            dp.push(vec![i32::MAX; ring.len()]);
        }

        let key: Vec<char> = key.chars().collect();
        for (index, value) in key.iter().enumerate() {
            let pos = map.get(&value).unwrap();

            if index == 0 {
                for i in pos {
                    dp[index][*i] = ((*i).min(ring.len() - *i) + 1) as i32;
                }
            } else {
                for i in pos {
                    let last = key[index - 1];
                    let last_pos = map.get(&last).unwrap();

                    for j in last_pos {
                        let now = *i as i32;
                        let last = *j as i32;

                        dp[index][*i] = dp[index][*i].min(
                            dp[index - 1][*j]
                                + (now - last)
                                    .abs()
                                    .min(ring.len() as i32 - (now - last).abs())
                                + 1,
                        )
                    }
                }
            }
        }

        *dp[key.len() - 1].iter().min().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_514() {
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
            4
        );
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "godding".to_string()),
            13
        );
    }
}
