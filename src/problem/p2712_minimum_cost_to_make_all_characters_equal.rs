/**
 * [2712] Minimum Cost to Make All Characters Equal
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let mut result = 0;
        let s: Vec<u8> = s.bytes().collect();

        for i in 1..s.len() {
            if s[i] != s[i - 1] {
                result += i.min(s.len() - i) as i64;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2712() {
        assert_eq!(2, Solution::minimum_cost("0011".to_owned()));
        assert_eq!(9, Solution::minimum_cost("010101".to_owned()));
    }
}
