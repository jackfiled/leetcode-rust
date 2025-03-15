/**
 * [3110] Score of a String
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.bytes()
            .zip(s.bytes().skip(1))
            .map(|(first, second)| first.abs_diff(second) as i32)
            .sum::<i32>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3110() {
        assert_eq!(13, Solution::score_of_string("hello".to_owned()));
        assert_eq!(50, Solution::score_of_string("zaz".to_owned()));
    }
}
