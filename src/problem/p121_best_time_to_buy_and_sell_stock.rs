/**
 * [121] Best Time to Buy and Sell Stock
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut min_num = prices[0];

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                result = result.max(prices[i] - min_num);
            } else {
                if prices[i] < min_num {
                    min_num = prices[i];
                }
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
    fn test_121() {
        assert_eq!(1, Solution::max_profit(vec![1, 2]));
    }
}
