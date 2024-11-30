/**
 * [3232] Find if Digit Game Can Be Won
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        nums.into_iter()
            .map(|x| if x >= 10 { (0, x) } else { (x, 0) })
            .fold([(0, 0)], |acc, e| [(acc[0].0 + e.0, acc[0].1 + e.1)])
            .iter()
            .all(|x| x.0 != x.1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3232() {
        assert!(!Solution::can_alice_win(vec![1, 2, 3, 4, 10]));
        assert!(Solution::can_alice_win(vec![1, 2, 3, 4, 5, 14]));
        assert!(Solution::can_alice_win(vec![5, 5, 5, 25]));
    }
}
