/**
 * [392] Is Subsequence
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let t: Vec<char> = t.chars().collect();

        let mut pos = 0;
        for c in s.chars() {
            while pos < t.len() && t[pos] != c {
                pos += 1;
            }

            if pos >= t.len() {
                return false;
            }

            pos += 1;
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_392() {
        assert!(Solution::is_subsequence(
            "abc".to_owned(),
            "ahbgdc".to_owned()
        ));
    }
}
