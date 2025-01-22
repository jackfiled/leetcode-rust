/**
 * [1561] Maximum Number of Coins You Can Get
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        let n = piles.len() / 3;
        piles.sort_unstable_by(|a, b| b.cmp(a));

        let mut result = 0;

        for i in (0..n).map(|x| x * 2 + 1) {
            result += piles[i]
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1561() {
        assert_eq!(9, Solution::max_coins(vec![2, 4, 1, 2, 7, 8]));
        assert_eq!(4, Solution::max_coins(vec![2, 4, 5]));
        assert_eq!(18, Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]));
    }
}
