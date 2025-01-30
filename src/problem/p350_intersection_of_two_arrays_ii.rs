/**
 * [350] Intersection of Two Arrays II
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();

        for i in nums1 {
            let entry = map.entry(i).or_insert((0, 0));
            entry.0 += 1;
        }

        for i in nums2 {
            if let Some(entry) = map.get_mut(&i) {
                entry.1 += 1;
            }
        }

        let mut result = vec![];
        for (key, value) in map.iter().filter_map(|(key, value)| {
            if value.1 != 0 {
                Some((*key, value.0.min(value.1)))
            } else {
                None
            }
        }) {
            for _ in 0..value {
                result.push(key)
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
    fn test_350() {
        assert_eq!(
            vec![2, 2],
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])
        );
    }
}
