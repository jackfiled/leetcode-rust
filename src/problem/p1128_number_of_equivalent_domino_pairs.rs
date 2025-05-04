/**
 * [1128] Number of Equivalent Domino Pairs
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        let mut result = 0;

        for card in dominoes.iter() {
            let sum = card[0] * 10 + card[1];

            if let Some(count) = map.get(&sum) {
                result += *count;
            }

            let entry = map.entry(sum).or_insert(0);
            *entry += 1;

            let reverse_sum = card[1] * 10 + card[0];
            if reverse_sum != sum {
                let entry = map.entry(card[1] * 10 + card[0]).or_insert(0);
                *entry += 1;
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
    fn test_1128() {
        assert_eq!(
            1,
            Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]])
        );
        assert_eq!(
            3,
            Solution::num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2]
            ])
        );
    }
}
