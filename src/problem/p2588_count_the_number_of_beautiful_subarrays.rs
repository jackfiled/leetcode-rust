/**
 * [2588] Count the Number of Beautiful Subarrays
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut prefix = vec![0; n + 1];

        for i in 0..n {
            prefix[i + 1] = prefix[i] ^ nums[i];
        }

        let mut map = HashMap::new();

        let mut result = 0;
        for i in 0..=n {
            if let Some(count) = map.get(&prefix[i]) {
                result += count;
            }

            let entry = map.entry(prefix[i] ^ 0).or_insert(0);
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
    fn test_2588() {
        assert_eq!(2, Solution::beautiful_subarrays(vec![4, 3, 1, 2, 4]));
        assert_eq!(0, Solution::beautiful_subarrays(vec![1, 10, 4]));
    }
}
