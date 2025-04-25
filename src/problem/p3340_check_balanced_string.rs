/**
 * [3340] Check Balanced String
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let num: Vec<i32> = num.bytes().map(|x| (x - b'0') as i32).collect();

        num.iter().step_by(2).sum::<i32>() == num.iter().skip(1).step_by(2).sum::<i32>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3340() {
        assert!(!Solution::is_balanced("1234".to_owned()));
        assert!(Solution::is_balanced("24123".to_owned()));
    }
}
