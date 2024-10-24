/**
 * [290] Word Pattern
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split(' ').collect();

        if words.len() != pattern.len() {
            return false;
        }

        let mut map = HashMap::new();
        let mut reverse_map = HashMap::new();

        for (c, word) in pattern.chars().zip(words) {
            let entry = map.entry(c).or_insert(word);

            if *entry != word {
                return false;
            }

            let entry = reverse_map.entry(word).or_insert(c);

            if *entry != c {
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
    fn test_290() {
        assert!(Solution::word_pattern(
            "abba".to_owned(),
            "dog cat cat dog".to_owned()
        ));
    }
}
