/**
 * [638] Shopping Offers
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let n = price.len();

        // 过滤出有优惠的大礼包
        let special: Vec<Vec<i32>> = special
            .into_iter()
            .filter_map(|s| {
                (price
                    .iter()
                    .enumerate()
                    .map(|(i, v)| s[i] * *v)
                    .sum::<i32>()
                    > s[n])
                    .then_some(s)
            })
            .collect();

        let mut memory = HashMap::new();

        Self::dfs(&price, &special, needs, &mut memory)
    }

    fn dfs(
        price: &Vec<i32>,
        special: &Vec<Vec<i32>>,
        current_needs: Vec<i32>,
        memory: &mut HashMap<Vec<i32>, i32>,
    ) -> i32 {
        if let Some(&v) = memory.get(&current_needs) {
            return v;
        }

        let n = price.len();
        let mut min_price: i32 = current_needs
            .iter()
            .enumerate()
            .map(|(i, &v)| price[i] * v)
            .sum();

        for s in special.iter() {
            let special_price = s[n];

            let mut next_need = Vec::with_capacity(n);
            for (i, &v) in current_needs.iter().enumerate() {
                if s[i] > v {
                    break;
                }

                next_need.push(v - s[i]);
            }

            // 可以购买大礼包
            if next_need.len() == n {
                min_price =
                    min_price.min(Self::dfs(price, special, next_need, memory) + special_price);
            }
        }

        memory.insert(current_needs, min_price);
        min_price
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_638() {
        assert_eq!(
            14,
            Solution::shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2])
        );
        assert_eq!(
            11,
            Solution::shopping_offers(
                vec![2, 3, 4],
                vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
                vec![1, 2, 1]
            )
        );
    }
}
