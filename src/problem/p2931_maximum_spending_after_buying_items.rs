/**
 * [2931] Maximum Spending After Buying Items
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Item {
    price: i64,
    shop: usize,
    pos: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.price.cmp(&self.price)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (values.len(), values[0].len());
        let mut heap = BinaryHeap::with_capacity(values.len());
        let mut result = 0;

        for i in 0..m {
            heap.push(Item {
                price: values[i][n - 1] as i64,
                shop: i,
                pos: n - 1,
            });
        }

        let mut day = 1;
        while let Some(top) = heap.pop() {
            result += top.price * day;
            day += 1;

            if top.pos > 0 {
                heap.push(Item {
                    price: values[top.shop][top.pos - 1] as i64,
                    shop: top.shop,
                    pos: top.pos - 1,
                })
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
    fn test_2931() {
        assert_eq!(
            285,
            Solution::max_spending(vec![vec![8, 5, 2], vec![6, 4, 1], vec![9, 7, 3]])
        );
    }
}
