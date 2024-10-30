/**
 * [3216] Lexicographically Smallest String After a Swap
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut s: Vec<u32> = s.chars().map(|x| x.to_digit(10).unwrap()).collect();

        for i in 0..s.len() - 1 {
            if s[i] % 2 != s[i + 1] % 2 {
                continue;
            }

            if s[i] > s[i + 1] {
                let temp = s[i];
                s[i] = s[i + 1];
                s[i + 1] = temp;
                break;
            }
        }

        s.into_iter().map(|x| x.to_string()).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3216() {
        assert_eq!(
            "43520".to_owned(),
            Solution::get_smallest_string("45320".to_owned())
        );
        assert_eq!(
            "001".to_owned(),
            Solution::get_smallest_string("001".to_owned())
        );
    }
}
