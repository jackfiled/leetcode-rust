/**
 * [2744] Find Maximum Number of String Pairs
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;

        for i in 0..words.len() {
            let word: String = words[i].chars().rev().collect();

            for j in (i + 1)..words.len() {
                if word == words[j] {
                    result += 1;
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2744() {
    }
}
