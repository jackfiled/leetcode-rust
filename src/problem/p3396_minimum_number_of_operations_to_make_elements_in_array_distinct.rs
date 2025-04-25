/**
 * [3396] Minimum Number of Operations to Make Elements in Array Distinct
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for &i in nums.iter() {
            let mut entry = map.entry(i).or_insert(0);
            *entry += 1;
        }

        let mut result = 0;
        let mut pos = 0;

        while pos < nums.len() {
            if map.values().all(|i| *i == 1) {
                break;
            }

            for p in pos..(pos + 3).min(nums.len()) {
                pos = p;
                let value = map.get_mut(&nums[pos]).unwrap();
                *value -= 1;
            }
            pos += 1;

            result += 1;
            map.retain(|_, v| *v != 0);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3396() {
        assert_eq!(2, Solution::minimum_operations(vec![5, 7, 11, 12, 12]));
        assert_eq!(
            2,
            Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7])
        );
        assert_eq!(2, Solution::minimum_operations(vec![4, 5, 6, 4, 4]));
        assert_eq!(0, Solution::minimum_operations(vec![6, 7, 8, 9]));
    }
}
