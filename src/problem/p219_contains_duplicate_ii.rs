/**
 * [219] Contains Duplicate II
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(last_pos) = map.get_mut(v) {
                let distance: i32 = (*last_pos - i as i32);
                let distance = distance.abs();

                if distance > 0 && distance <= k {
                    return true;
                } else {
                    *last_pos = i as i32;
                }
            } else {
                map.insert(*v, i as i32);
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_219() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ));
    }
}
