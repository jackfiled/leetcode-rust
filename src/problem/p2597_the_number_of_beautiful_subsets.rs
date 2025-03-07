/**
 * [2597] The Number of Beautiful Subsets
 */
pub struct Solution {}

// submission codes start here
use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();

        for i in nums {
            let value = i % k;
            let entry = map.entry(value).or_insert(BTreeMap::new());
            let group_entry = entry.entry(i).or_insert(0);
            *group_entry += 1;
        }

        let mut result = 1;

        for group in map.values() {
            let n = group.len();
            // (x, y) x表示不选择第i个数
            // y表示选择第i个数
            let mut dp = vec![(0, 0); n];
            // 懒得用迭代器移来移去
            // 直接复制到数组开干
            let array: Vec<(i32, i32)> = group.iter().map(|(x, y)| (*x, *y)).collect();

            for (i, &(key, count)) in array.iter().enumerate() {
                if i == 0 {
                    dp[0].0 = 1;
                    dp[0].1 = (1 << count) - 1;
                } else {
                    dp[i].0 = dp[i - 1].0 + dp[i - 1].1;

                    if key - array[i - 1].0 == k {
                        dp[i].1 = dp[i - 1].0 * ((1 << count) - 1);
                    } else {
                        dp[i].1 = (dp[i - 1].0 + dp[i - 1].1) * ((1 << count) - 1);
                    }
                }
            }

            result *= dp[n - 1].0 + dp[n - 1].1;
        }

        result - 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2597() {
        assert_eq!(4, Solution::beautiful_subsets(vec![2, 4, 6], 2));
        assert_eq!(1, Solution::beautiful_subsets(vec![1], 1));
    }
}
