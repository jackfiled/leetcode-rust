/**
 * [2070] Most Beautiful Item for Each Query
 */
pub struct Solution {}

// submission codes start here
use std::collections::BTreeMap;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut price_map = BTreeMap::new();

        for item in items.iter() {
            let entry = price_map.entry(item[0] as usize).or_insert(0);
            *entry = (*entry).max(item[1]);
        }

        let mut query_map: BTreeMap<usize, i32> = BTreeMap::new();
        let mut result = Vec::with_capacity(queries.len());

        for price in queries {
            let price = price as usize;
            let (min_price, mut min_result) = match query_map.range(..=price).last() {
                Some((&p, &r)) => (p, r),
                None => (0, 0),
            };

            if min_price == price {
                result.push(min_result);
                continue;
            }

            for (&p, &r) in price_map.range(min_price + 1..=price) {
                min_result = min_result.max(r);
                query_map.insert(p, min_result);
            }

            result.push(min_result);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2070() {
        assert_eq!(
            vec![2, 4, 5, 5, 6, 6],
            Solution::maximum_beauty(
                vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
                vec![1, 2, 3, 4, 5, 6]
            )
        );
        assert_eq!(
            vec![4],
            Solution::maximum_beauty(
                vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
                vec![1]
            )
        );
        assert_eq!(
            vec![0],
            Solution::maximum_beauty(vec![vec![10, 1000]], vec![5])
        );
    }
}
