/**
 * [3375] Minimum Operations to Make Array Values Equal to K
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();

        let mut result = 0;

        for &i in set.iter() {
            if i < k {
                return -1;
            }

            if i > k {
                result += 1;
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
    fn test_3375() {
        assert_eq!(2, Solution::min_operations(vec![5, 2, 5, 4, 5], 2));
        assert_eq!(-1, Solution::min_operations(vec![2, 1, 2], 2));
        assert_eq!(4, Solution::min_operations(vec![9, 7, 5, 3], 1));
    }
}
