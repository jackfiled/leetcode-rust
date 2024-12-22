/**
 * [1387] Sort Integers by The Power Value
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut memo = HashMap::new();
        memo.insert(1, 0);

        let mut array: Vec<(i32, i32)> =
            (lo..=hi).map(|x| (Self::search(x, &mut memo), x)).collect();
        array.sort_unstable();

        array[k as usize - 1].1
    }

    fn search(x: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&result) = memo.get(&x) {
            return result;
        }

        let result = if x % 2 == 0 {
            Self::search(x / 2, memo) + 1
        } else {
            Self::search((x * 3 + 1) / 2, memo) + 2
        };

        memo.insert(x, result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let mut memo = HashMap::new();
        memo.insert(1, 0);

        assert_eq!(16, Solution::search(7, &mut memo));
        assert_eq!(3, Solution::search(8, &mut memo));
        assert_eq!(19, Solution::search(9, &mut memo));
        assert_eq!(6, Solution::search(10, &mut memo));
        assert_eq!(14, Solution::search(11, &mut memo));
        assert_eq!(9, Solution::search(12, &mut memo));
        assert_eq!(9, Solution::search(13, &mut memo));
        assert_eq!(17, Solution::search(14, &mut memo));
        assert_eq!(17, Solution::search(15, &mut memo));
    }

    #[test]
    fn test_1387() {
        assert_eq!(13, Solution::get_kth(12, 15, 2));
        assert_eq!(7, Solution::get_kth(7, 11, 4));
    }
}
