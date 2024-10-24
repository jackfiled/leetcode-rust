/**
 * [3137] Minimum Number of Operations to Make Word K-Periodic
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut result = i32::MAX;
        let n = word.len();
        let k = k as usize;

        for i in (0..n).step_by(k) {
            let key = &word[i..i + k];

            let entry = map.entry(key).or_insert(0);
            *entry += 1;

            result = result.min((n / k) as i32 - *entry);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3137() {
        assert_eq!(
            1,
            Solution::minimum_operations_to_make_k_periodic("leetcodeleet".to_owned(), 4)
        );
    }
}
