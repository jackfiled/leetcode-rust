/**
 * [2549] Count Distinct Numbers on Board
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        return if n == 1 { 1 } else { n - 1 };
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2549() {}
}
