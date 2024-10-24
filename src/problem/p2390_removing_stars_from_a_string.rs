/**
 * [2390] Removing Stars From a String
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let s: Vec<char> = s.chars().collect();

        let mut result = Vec::with_capacity(s.len());

        for i in s {
            match i {
                '*' => {
                    result.pop();
                }
                _ => {
                    result.push(i);
                }
            }
        }

        result.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2390() {
        assert_eq!(
            "lecoe".to_owned(),
            Solution::remove_stars("leet**cod*e".to_owned())
        );
        assert_eq!(
            "".to_owned(),
            Solution::remove_stars("erase*****".to_owned())
        );
    }
}
