/**
 * [2272] Substring With Largest Variance
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let word: Vec<char> = s.chars().collect();
        let mut pos_map = HashMap::new();

        for (i, c) in word.into_iter().enumerate() {
            let entry = pos_map.entry(c).or_insert(vec![]);
            entry.push(i);
        }

        let mut result = 0;

        for (c0, pos0) in pos_map.iter() {
            for (c1, pos1) in pos_map.iter() {
                if c0 != c1 {
                    let (mut i, mut j) = (0, 0);
                    let (mut f, mut g) = (0, i32::MIN);

                    // 这里的算法类似于合并两个有序列表
                    while i < pos0.len() || j < pos1.len() {
                        if j == pos1.len() || (i < pos0.len() && pos0[i] < pos1[j]) {
                            (f, g) = (f.max(0) + 1, g + 1);
                            i += 1;
                        } else {
                            (f, g) = (f.max(0) - 1, f.max(g).max(0) - 1);
                            j += 1;
                        }

                        result = result.max(g);
                    }
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2272() {
        assert_eq!(3, Solution::largest_variance("aababbb".to_owned()));
        assert_eq!(0, Solution::largest_variance("abcde".to_owned()));
    }
}
