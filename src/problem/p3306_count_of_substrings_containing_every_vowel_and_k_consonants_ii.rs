/**
 * [3306] Count of Substrings Containing Every Vowel and K Consonants II
 */
pub struct Solution {}

// submission codes start here
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word: Vec<char> = word.chars().collect();
        let vowel_set = HashSet::from(['a', 'e', 'i', 'o', 'u']);

        let count = |m| -> i64 {
            let mut consonants = 0;
            let mut result = 0;
            let mut occurence = HashMap::new();

            let mut right = 0;
            for left in 0..word.len() {
                while right < word.len() && (consonants < m || occurence.len() < vowel_set.len()) {
                    if vowel_set.contains(&word[right]) {
                        let entry = occurence.entry(word[right]).or_insert(0i64);
                        *entry += 1;
                    } else {
                        consonants += 1;
                    }
                    right += 1;
                }

                if consonants >= m && occurence.len() == vowel_set.len() {
                    result += (word.len() - right + 1) as i64;
                }

                if vowel_set.contains(&word[left]) {
                    let v = occurence.get_mut(&word[left]).unwrap();
                    *v -= 1;
                    if *v == 0 {
                        occurence.remove(&word[left]);
                    }
                } else {
                    consonants -= 1;
                }
            }

            result
        };

        count(k) - count(k + 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3306() {
        assert_eq!(0, Solution::count_of_substrings("aeioqq".to_owned(), 1));
    }
}
