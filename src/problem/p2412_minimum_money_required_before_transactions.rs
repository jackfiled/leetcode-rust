/**
 * [2412] Minimum Money Required Before Transactions
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        // (total_lose, max(cost, cashback))
        let (total_lose, mx) = transactions.into_iter().fold((0, 0), |(total_lose, m), v| {
            let (cost, cash_back) = (v[0] as i64, v[1] as i64);
            (
                total_lose + 0.max(cost - cash_back),
                m.max(cost.min(cash_back)),
            )
        });

        total_lose + mx
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2412() {
        assert_eq!(
            10,
            Solution::minimum_money(vec![vec![2, 1], vec![5, 0], vec![4, 2]])
        );
        assert_eq!(3, Solution::minimum_money(vec![vec![3, 0], vec![0, 3]]));
    }
}
