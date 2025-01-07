/**
 * [3019] Number of Changing Keys
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut second_chars = s.chars();
        second_chars.next();

        second_chars
            .zip(s.chars())
            .filter_map(|(a, b)| {
                if a.to_lowercase().next() != b.to_lowercase().next() {
                    Some(())
                } else {
                    None
                }
            })
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3019() {
        assert_eq!(2, Solution::count_key_changes("aAbBcC".to_owned()));
        assert_eq!(0, Solution::count_key_changes("AaAaAaaA".to_owned()));
    }
}
