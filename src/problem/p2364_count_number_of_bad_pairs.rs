/**
 * [2364] Count Number of Bad Pairs
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len() as i64;
        let mut map = HashMap::new();

        for (i, v) in nums.into_iter().enumerate() {
            let delta = v - i as i32;
            let mut entry = map.entry(delta).or_insert(0);
            *entry += 1;
        }

        // 反过来计算好数对的数量
        let good_pair_count = map
            .values()
            .filter_map(|&v| if v >= 2 { Some(v * (v - 1) / 2) } else { None })
            .sum::<i64>();

        n * (n - 1) / 2 - good_pair_count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2364() {
        assert_eq!(5, Solution::count_bad_pairs(vec![4, 1, 3, 3]));
        assert_eq!(0, Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]));
    }
}
