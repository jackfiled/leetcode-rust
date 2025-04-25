/**
 * [2843]   Count Symmetric Integers
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        // 10 -> 99 1000 -> 9999
        (low..=high)
            .filter_map(|i| {
                if i >= 10 && i <= 99 {
                    if i / 10 == i % 10 {
                        return Some(());
                    }
                }

                if i >= 1000 && i <= 9999 {
                    if i / 1000 + (i / 100) % 10 == (i / 10) % 10 + i % 10 {
                        return Some(());
                    }
                }

                None
            })
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2843() {
        assert_eq!(9, Solution::count_symmetric_integers(1, 99));
        assert_eq!(4, Solution::count_symmetric_integers(1200, 1230));
    }
}
