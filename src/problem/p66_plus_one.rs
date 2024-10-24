/**
 * [66] Plus One
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut overflow = 1;

        for i in (0..digits.len()).rev() {
            let bit = digits[i] + overflow;
            digits[i] = bit % 10;
            overflow = bit / 10;
        }

        if overflow > 0 {
            digits.insert(0, overflow);
        }

        digits
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_66() {
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
    }
}
