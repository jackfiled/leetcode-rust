/**
 * [2278] Percentage of Letter in String
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.chars().filter(|c| *c == letter).count() * 100 / s.len()) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2278() {
        assert_eq!(33, Solution::percentage_letter("foobar".to_owned(), 'o'));
        assert_eq!(0, Solution::percentage_letter("jjjj".to_owned(), 'k'));
    }
}
