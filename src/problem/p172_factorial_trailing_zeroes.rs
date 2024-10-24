/**
 * [172] Factorial Trailing Zeroes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;

        while n > 0 {
            n /= 5;
            result += n;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_172() {}
}
