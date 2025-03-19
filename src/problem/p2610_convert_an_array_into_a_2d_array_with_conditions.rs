/**
 * [2610] Convert an Array Into a 2D Array With Conditions
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut map = HashMap::new();

        for i in nums {
            let entry = map.entry(i).or_insert(0);
            *entry += 1;
        }

        while !map.is_empty() {
            let mut row = Vec::with_capacity(map.len());
            for (k, v) in map.iter_mut() {
                row.push(*k);
                *v -= 1;
            }

            map.retain(|_, &mut x| x != 0);
            result.push(row);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2610() {
        assert_eq!(
            vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]],
            Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1])
        );
    }
}
