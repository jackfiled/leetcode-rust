/**
 * [122] Best Time to Buy and Sell Stock II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut once_buy = 0;
        let mut min_buy = prices[0];

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                once_buy = once_buy.max(prices[i] - min_buy);
            } else if prices[i] < prices[i - 1] {
                result += once_buy;
                once_buy = 0;
                min_buy = prices[i];
            }
        }

        if once_buy != 0 {
            result += once_buy;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_122() {
        assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 2, 1]));
    }
}
