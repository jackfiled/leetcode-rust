/**
 * [17] Letter Combinations of a Phone Number
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = vec![];
        let digits: Vec<usize> = digits
            .chars()
            .map(|c| (c.to_digit(10).unwrap() - 2) as usize)
            .collect();
        let mut str = Vec::with_capacity(digits.len());

        Self::search(&digits, &mut str, &mut result, 0);

        result
    }

    fn search(digits: &Vec<usize>, str: &mut Vec<char>, result: &mut Vec<String>, pos: usize) {
        let map = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];

        if pos >= digits.len() {
            if !str.is_empty() {
                result.push(str.iter().collect());
            }
            return;
        }

        for &c in &map[digits[pos]] {
            str.push(c);
            Self::search(digits, str, result, pos + 1);
            str.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {}
}
