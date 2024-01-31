/**
 * [2670] Find the Distinct Difference Array
 */
pub struct Solution {}


// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut right = HashMap::new();

        for i in &nums {
            let entry = right.entry(*i).or_insert(0);
            *entry += 1;
        }

        let mut result = Vec::with_capacity(nums.len());
        let mut left = HashMap::new();

        for i in &nums {
            let left_entry = left.entry(*i).or_insert(0);
            *left_entry += 1;

            let right_entry = right.get_mut(i).unwrap();
            *right_entry -= 1;

            if *right_entry == 0 {
                right.remove(i);
            }

            result.push(left.len() as i32 - right.len() as i32);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2670() {
        assert_eq!(Solution::distinct_difference_array(vec![1, 2, 3, 4, 5]),
                   vec![-3, -1, 1, 3, 5]);
    }
}
