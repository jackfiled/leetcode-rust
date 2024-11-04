/**
 * [633] Sum of Square Numbers
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let mut i = 0i64;
        let c = c as i64;

        while i.pow(2) <= c {
            let value = c - i.pow(2);

            set.insert(i.pow(2));
            if set.contains(&value) {
                return true;
            }

            i += 1;
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_633() {
        assert!(Solution::judge_square_sum(5));
        assert!(Solution::judge_square_sum(4));
        assert!(!Solution::judge_square_sum(3));
        assert!(Solution::judge_square_sum(2));
    }
}
