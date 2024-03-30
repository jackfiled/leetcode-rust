/**
 * [2952] Minimum Number of Coins to be Added
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        let mut coins = coins;
        coins.sort_unstable();

        let mut result = 0;
        let mut i = 0;
        let mut x = 1;

        while x <= target {
            if i < coins.len() && coins[i] <= x {
                x += coins[i];
                i += 1;
            } else {
                x = x * 2;
                result += 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2952() {
        assert_eq!(2, Solution::minimum_added_coins(vec![1, 4, 10], 19));
        assert_eq!(
            0,
            Solution::minimum_added_coins(vec![1, 2, 4, 6, 7, 9, 9, 10], 48)
        );
    }
}
