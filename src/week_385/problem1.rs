pub struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        let words: Vec<Vec<char>> = words.iter()
            .map(|s| {s.chars().collect()})
            .collect();
        
        for j in 1..words.len() {
            for i in 0..j {
                if Solution::is_prefix_and_suffix(&words[i], &words[j]) {
                    result += 1;
                }
            }
        }

        result
    }

    fn is_prefix_and_suffix(str1: &Vec<char>, str2: &Vec<char>) -> bool {
        if str1.len() > str2.len() {
            return false;
        }

        let mut prefix = 0;
        let mut suffix = str2.len() - str1.len();

        for c in str1 {
            if *c != str2[prefix] || *c != str2[suffix] {
                return false;
            }
            prefix += 1;
            suffix += 1;
        }

        true
    }
}