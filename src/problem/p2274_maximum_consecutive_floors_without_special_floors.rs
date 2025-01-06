/**
 * [2274] Maximum Consecutive Floors Without Special Floors
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        special.sort_unstable();

        let mut result = special.first().unwrap() - bottom;
        result = result.max(top - special.last().unwrap());

        for i in 0..special.len() - 1 {
            result = result.max(special[i + 1] - special[i] - 1);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2274() {
        assert_eq!(3, Solution::max_consecutive(2, 9, vec![4, 6]));
        assert_eq!(0, Solution::max_consecutive(6, 8, vec![7, 6, 8]));
    }
}
