/**
 * [242] Valid Anagram
 */
pub struct Solution {}


// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = HashMap::new();

        for c in s.chars() {
            let entry = map.entry(c).or_insert(0);

            *entry += 1;
        }

        for c in t.chars() {
            match map.get_mut(&c) {
                Some(value) => {
                    *value -= 1;

                    if *value == 0 {
                        map.remove(&c);
                    }
                },
                None => {
                    return false;
                }
            }
        }

        map.len() == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert!(Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()));
    }
}
