/**
 * [3305] Count of Substrings Containing Every Vowel and K Consonants I
 */
pub struct Solution {}

// submission codes start here
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        let word: Vec<char> = word.chars().collect();
        let vowel_set = HashSet::from(['a', 'e', 'i', 'o', 'u']);

        let count = |m| -> i32 {
            let mut consonants = 0;
            let mut result = 0;
            let mut occurence = HashMap::new();

            let mut right = 0;
            for left in 0..word.len() {
                while right < word.len() && (consonants < m || occurence.len() < vowel_set.len()) {
                    if vowel_set.contains(&word[right]) {
                        let entry = occurence.entry(word[right]).or_insert(0);
                        *entry += 1;
                    } else {
                        consonants += 1;
                    }
                    right += 1;
                }

                if consonants >= m && occurence.len() == vowel_set.len() {
                    result += (word.len() - right + 1) as i32;
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
    fn test_3305() {
        assert_eq!(3, Solution::count_of_substrings("auieoui".to_owned(), 0));
        assert_eq!(2, Solution::count_of_substrings("ieiaoud".to_owned(), 0));
        assert_eq!(3, Solution::count_of_substrings("iqeaouqi".to_owned(), 2));
        assert_eq!(
            3,
            Solution::count_of_substrings("ieaouqqieaouqq".to_owned(), 1)
        );
        assert_eq!(0, Solution::count_of_substrings("aeioqq".to_owned(), 1));
        assert_eq!(1, Solution::count_of_substrings("aeiou".to_owned(), 0));
    }
}
