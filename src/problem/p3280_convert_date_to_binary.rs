/**
 * [3280] Convert Date to Binary
 */
pub struct Solution {}

// submission codes start here
use std::str::FromStr;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .map(|x| u32::from_str(x).unwrap())
            .map(|mut x| {
                let mut result = vec![];

                while x != 0 {
                    result.push(x % 2);
                    x = x / 2;
                }

                result
                    .iter()
                    .rev()
                    .map(|c| c.to_string())
                    .collect::<String>()
            })
            .reduce(|acc, e| acc + "-" + e.as_str())
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3280() {
        assert_eq!(
            "100000100000-10-11101".to_owned(),
            Solution::convert_date_to_binary("2080-02-29".to_owned())
        );
        assert_eq!(
            "11101101100-1-1".to_owned(),
            Solution::convert_date_to_binary("1900-01-01".to_owned())
        );
    }
}
