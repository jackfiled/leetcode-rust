/**
 * [2788] Split Strings by Separator
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = Vec::new();

        for s in &words {
            for i in s.split(separator) {
                let word = String::from(i);

                if word.is_empty() {
                    continue;
                }

                result.push(word)
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
    fn test_2788() {
    }
}
