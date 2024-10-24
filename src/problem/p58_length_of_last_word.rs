/**
 * [58] Length of Last Word
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.split(' ').collect();

        for word in words.iter().rev() {
            if word.len() != 0 {
                return word.len() as i32;
            }
        }

        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_58() {
        assert_eq!(5, Solution::length_of_last_word("Hello World".to_owned()));
        assert_eq!(
            4,
            Solution::length_of_last_word("  fly me   to the moon  ".to_owned())
        );
        assert_eq!(
            6,
            Solution::length_of_last_word("luffy is still joyboy".to_owned())
        );
    }
}
