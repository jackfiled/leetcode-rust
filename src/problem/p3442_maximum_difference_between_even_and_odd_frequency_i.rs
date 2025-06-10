/**
 * [3442] Maximum Difference Between Even and Odd Frequency I
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut frequency_map = HashMap::new();

        for c in s.bytes() {
            let entry = frequency_map.entry(c).or_insert(0);
            *entry += 1;
        }

        let mut odd_max = 0;
        let mut even_min = i32::MAX;

        for &v in frequency_map.values() {
            if v % 2 == 0 {
                even_min = even_min.min(v);
            } else {
                odd_max = odd_max.max(v);
            }
        }

        odd_max - even_min
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3442() {
        assert_eq!(3, Solution::max_difference("aaaaabbc".to_string()));
        assert_eq!(1, Solution::max_difference("abcabcab".to_string()));
    }
}
