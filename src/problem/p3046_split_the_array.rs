/**
 * [3046] Split the Array
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut map = HashMap::with_capacity(nums.len());

        for i in nums {
            let entry = map.entry(i).or_insert(0);
            *entry += 1;

            if *entry > 2 {
                return false;
            }
        }

        map.iter()
            .filter_map(|(_, v)| if *v == 1 { Some(()) } else { None })
            .count()
            % 2
            == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3046() {
        assert!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]));
        assert!(Solution::is_possible_to_split(vec![4, 4, 9, 10]));
        assert!(!Solution::is_possible_to_split(vec![1, 1, 1, 1]));
    }
}
