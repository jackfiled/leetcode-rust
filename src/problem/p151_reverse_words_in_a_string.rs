/**
 * [151] Reverse Words in a String
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words = Vec::new();

        for word in s.split(' ') {
            if word.len() != 0 {
                words.push(word);
            }
        }
        
        let length = words.len();
        let mut result = String::from(words[length - 1]);

        for i in (0..length - 1).rev() {
            result.push(' ');
            result.push_str(words[i]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_151() {
        assert_eq!("blue is sky the".to_owned(), Solution::reverse_words("the sky is blue".to_owned()));
    }
}
