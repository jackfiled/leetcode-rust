/**
 * [67] Add Binary
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<u32> = a.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
        let b: Vec<u32> = b.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();

        let mut result: Vec<u32> = Vec::with_capacity(a.len().max(b.len()) + 2);

        let mut overflow = 0;
        for i in 0..a.len().max(b.len()) {
            let mut bit = overflow;

            if i < a.len() {
                bit += a[i];
            }
            if i < b.len() {
                bit += b[i];
            }

            match bit {
                3 => {
                    result.push(1);
                    overflow = 1
                }
                2 => {
                    result.push(0);
                    overflow = 1
                }
                1 => {
                    result.push(1);
                    overflow = 0
                }
                0 => {
                    result.push(0);
                    overflow = 0
                }
                _ => {}
            }
        }

        if overflow != 0 {
            result.push(overflow);
        }

        if result.len() == 0 {
            result.push(0);
        }

        result.iter().rev().map(|i| i.to_string()).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67() {
        assert_eq!(
            "100".to_owned(),
            Solution::add_binary("11".to_owned(), "1".to_owned())
        );
        assert_eq!(
            "10101".to_owned(),
            Solution::add_binary("1010".to_owned(), "1011".to_owned())
        );
        assert_eq!(
            "110110".to_owned(),
            Solution::add_binary("100".to_owned(), "110010".to_owned())
        );
    }
}
