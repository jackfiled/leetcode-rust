/**
 * [2275] Largest Combination With Bitwise AND Greater Than Zero
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 0..31 {
            result = result.max(
                candidates
                    .iter()
                    .filter_map(|&x| if x & (1 << i) > 0 { Some(()) } else { None })
                    .count(),
            );
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2275() {
        assert_eq!(
            4,
            Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14])
        );
        assert_eq!(2, Solution::largest_combination(vec![8, 8]));
    }
}
