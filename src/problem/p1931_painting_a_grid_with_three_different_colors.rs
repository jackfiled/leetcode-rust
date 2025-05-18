/**
 * [1931] Painting a Grid With Three Different Colors
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let mut mask_count = 3_usize.pow(m as u32);
        // 存储一行的所有合法涂色方式
        // key表示mask
        // value表示实际的涂色数组
        let mut valid_masks = HashMap::new();

        for mask in 0..mask_count {
            let mut color = vec![];
            let mut m_mask = mask;

            for _ in 0..m {
                color.push(m_mask % 3);
                m_mask = m_mask / 3;
            }

            if color.iter().skip(1).zip(color.iter()).all(|(x, y)| x != y) {
                valid_masks.insert(mask, color);
            }
        }

        // 预处理相邻行的合法表示
        let mut adjacent_valid_masks = HashMap::new();

        for (&mask1, color1) in valid_masks.iter() {
            for (&mask2, color2) in valid_masks.iter() {
                if color1.iter().zip(color2.iter()).all(|(x, y)| x != y) {
                    let entry = adjacent_valid_masks.entry(mask1).or_insert(vec![]);
                    entry.push(mask2)
                }
            }
        }

        // 滚动的DP数组
        let mut dp = vec![0; mask_count];

        // 初始化为第一行
        for &mask in valid_masks.keys() {
            dp[mask] = 1;
        }

        for _ in 1..n {
            let mut next_dp = vec![0; mask_count];

            for &next_mask in valid_masks.keys() {
                for &mask in adjacent_valid_masks.get(&next_mask).unwrap() {
                    next_dp[next_mask] += dp[mask];

                    if next_dp[next_mask] >= MOD {
                        next_dp[next_mask] -= MOD;
                    }
                }
            }

            dp = next_dp;
        }

        dp.into_iter().fold(0, |sum, i| {
            if sum + i >= MOD {
                sum + i - MOD
            } else {
                sum + i
            }
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1931() {
        assert_eq!(3, Solution::color_the_grid(1, 1));
        assert_eq!(6, Solution::color_the_grid(1, 2));
        assert_eq!(580986, Solution::color_the_grid(5, 5));
    }
}
