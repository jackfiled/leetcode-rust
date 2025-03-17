/**
 * [1963] Minimum Number of Swaps to Make the String Balanced
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut count = 0;
        let mut min_count = 0;

        for c in s.chars() {
            match c {
                '[' => count += 1,
                ']' => {
                    count -= 1;
                    min_count = min_count.min(count)
                }
                _ => {}
            }
        }

        (-min_count + 1) / 2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1963() {
        assert_eq!(1, Solution::min_swaps("][][".to_owned()));
        assert_eq!(2, Solution::min_swaps("]]][[[".to_owned()));
        assert_eq!(0, Solution::min_swaps("[]".to_owned()));
    }
}
