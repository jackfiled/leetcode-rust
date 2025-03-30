/**
 * [2109] Adding Spaces to a String
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut result = Vec::with_capacity(s.len() + spaces.len());
        let mut iter = spaces.into_iter().peekable();

        for (i, v) in s.into_iter().enumerate() {
            if let Some(&pos) = iter.peek() {
                if i == pos as usize {
                    iter.next();
                    result.push(' ');
                }
            }

            result.push(v);
        }

        result.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2109() {
        assert_eq!(
            "Leetcode Helps Me Learn",
            Solution::add_spaces("LeetcodeHelpsMeLearn".to_owned(), vec![8, 13, 15])
        );
        assert_eq!(
            "i code in py thon",
            Solution::add_spaces("icodeinpython".to_owned(), vec![1, 5, 7, 9])
        );
        assert_eq!(
            " s p a c i n g",
            Solution::add_spaces("spacing".to_owned(), vec![0, 1, 2, 3, 4, 5, 6])
        );
    }
}
