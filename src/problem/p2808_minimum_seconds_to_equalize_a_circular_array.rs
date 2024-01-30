/**
 * [2808] Minimum Seconds to Equalize a Circular Array
 */
pub struct Solution {}


// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let entry = map.entry(*num).or_insert(Vec::new());
            entry.push(index);
        }

        let mut result = i32::MAX;

        for (_, pos) in map {
            let mut max_distance = pos[0] + nums.len() - pos.last().unwrap();

            for i in 1..pos.len() {
                max_distance = max_distance.max(pos[i] - pos[i - 1]);
            }

            result = result.min(max_distance as i32 / 2);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2808() {
        assert_eq!(Solution::minimum_seconds(vec![1,2,1,2]), 1);
        assert_eq!(Solution::minimum_seconds(vec![2,1,3,3,2]), 2);
    }
}
