/**
 * [205] Isomorphic Strings
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut inverse_map = HashMap::with_capacity(s.len());
        let mut reverse_map = HashMap::with_capacity(t.len());

        for (c1, c2) in s.chars().zip(t.chars()) {
            let entry = inverse_map.entry(c1).or_insert(c2);

            if *entry != c2 {
                return false;
            }

            let entry = reverse_map.entry(c2).or_insert(c1);

            if *entry != c1 {
                return false;
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_205() {}
}
