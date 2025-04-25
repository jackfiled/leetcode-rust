/**
 * [2845] Count of Interesting Subarrays
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let n = nums.len();
        let mut count = vec![0; n + 1];
        for (i, &v) in nums.iter().enumerate() {
            count[i + 1] = count[i] + if v % modulo == k { 1 } else { 0 };
        }

        let mut map = HashMap::new();
        let mut result = 0;

        // 类似于两数之和
        for i in 0..=n {
            let target = (count[i] + modulo - k) % modulo;

            if let Some(&v) = map.get(&target) {
                result += v;
            }

            let entry = map.entry(count[i] % modulo).or_insert(0);
            *entry += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2845() {
        assert_eq!(
            3,
            Solution::count_interesting_subarrays(vec![3, 2, 4], 2, 1)
        );
        assert_eq!(
            2,
            Solution::count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0)
        );
    }
}
