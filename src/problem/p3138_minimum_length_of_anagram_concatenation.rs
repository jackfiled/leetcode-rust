/**
 * [3138] Minimum Length of Anagram Concatenation
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let length = s.len();

        for i in 1..length {
            if length % i != 0 {
                continue;
            }

            let count = length / i;
            let mut standard_map = HashMap::new();

            for k in 0..i {
                let entry = standard_map.entry(s[k]).or_insert(0);
                *entry += 1;
            }

            let mut flag = true;
            for j in 1..count {
                let mut map = HashMap::new();
                for k in (j * i)..(i * (j + 1)) {
                    let entry = map.entry(s[k]).or_insert(0);
                    *entry += 1;
                }

                flag = map == standard_map;
                if !flag {
                    break;
                }
            }

            if flag {
                return i as i32;
            }
        }

        s.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3138() {
        assert_eq!(2, Solution::min_anagram_length("abba".to_owned()));
        assert_eq!(4, Solution::min_anagram_length("cdef".to_owned()));
    }
}
