/**
 * [123] Best Time to Buy and Sell Stock III
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let (mut buy1, mut sell1) = (-prices[0], 0);
        let (mut buy2, mut sell2) = (-prices[0], 0);

        for i in 1..n {
            buy1 = buy1.max(-prices[i]);
            sell1 = sell1.max(buy1 + prices[i]);
            buy2 = buy2.max(sell1 - prices[i]);
            sell2 = sell2.max(buy2 + prices[i]);
        }

        sell2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
        assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    }
}
