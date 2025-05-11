/**
 * [1550] Three Consecutive Odds
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }

        arr.iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i, &z)| {
                if arr[i - 2] % 2 == 1 && arr[i - 1] % 2 == 1 && z % 2 == 1 {
                    Some(())
                } else {
                    None
                }
            })
            .next()
            .is_some()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1550() {
        assert!(!Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
        assert!(Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7, 23, 12
        ]));
    }
}
